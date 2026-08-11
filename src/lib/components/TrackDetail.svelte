<script lang="ts">
  import { api } from "../api";
  import { player } from "../stores/player.svelte";
  import { likes } from "../stores/likes.svelte";
  import { formatDuration, handleOf, isPlayable } from "../types";
  import type { Comment, Profile, Track } from "../types";
  import Icon from "./Icon.svelte";
  import ShareButton from "./ShareButton.svelte";
  import FollowButton from "./FollowButton.svelte";
  import AddToPlaylistModal from "./AddToPlaylistModal.svelte";
  import TrackCard from "./TrackCard.svelte";
  import { openUrl } from "@tauri-apps/plugin-opener";

  let {
    track,
    onBack,
    onOpenProfile,
    onOpenTrack,
    me = null,
  }: {
    track: Track;
    onBack: () => void;
    onOpenProfile: (id: number) => void;
    onOpenTrack?: (t: Track) => void;
    me?: Profile | null;
  } = $props();

  let isLiked = $derived(likes.has(track.id));
  let isCurrent = $derived(player.current?.id === track.id);
  let isPlaying = $derived(isCurrent && player.isPlaying);
  let playable = $derived(isPlayable(track));

  let reposted = $state(false);
  let repostBusy = $state(false);
  let showAddToPlaylist = $state(false);

  let comments = $state<Comment[]>([]);
  let commentsLoading = $state(true);
  let commentsError = $state("");
  let newComment = $state("");
  let postingComment = $state(false);

  $effect(() => {
    commentsLoading = true;
    commentsError = "";
    comments = [];
    api
      .trackComments(track.id)
      .then((c) => (comments = c))
      .catch((e) => (commentsError = `Failed to load comments: ${e}`))
      .finally(() => (commentsLoading = false));
  });

  // Both of these are supplementary/recommendation content, not core to the
  // page -- a failure just means an empty shelf (silently logged), not a
  // user-facing error, matching Profile.svelte's own userTracks fetch.
  let relatedTracks = $state<Track[]>([]);
  let relatedLoading = $state(true);

  $effect(() => {
    relatedLoading = true;
    relatedTracks = [];
    api
      .relatedTracks(track.id)
      .then((t) => (relatedTracks = t))
      .catch((e) => console.error("failed to load related tracks", e))
      .finally(() => (relatedLoading = false));
  });

  let moreByArtist = $state<Track[]>([]);
  let moreByArtistLoading = $state(true);

  $effect(() => {
    const userId = track.user?.id;
    if (!userId) {
      moreByArtist = [];
      moreByArtistLoading = false;
      return;
    }
    moreByArtistLoading = true;
    moreByArtist = [];
    api
      .userTracks(userId)
      .then((t) => (moreByArtist = t.filter((other) => other.id !== track.id)))
      .catch((e) => console.error("failed to load more tracks by artist", e))
      .finally(() => (moreByArtistLoading = false));
  });

  // The connect -> write -> update-store flow lives on the shared `likes`
  // store (see its doc comment) -- this just decides what to do on
  // decline/failure, same graceful-degradation pattern as FollowButton.
  async function toggleLike() {
    try {
      if ((await likes.toggle(track)) === "declined" && track.permalink_url) {
        openUrl(track.permalink_url);
      }
    } catch (e) {
      commentsError = `Failed to ${isLiked ? "unlike" : "like"} track: ${e}`;
    }
  }

  async function toggleRepost() {
    if (repostBusy) return;
    repostBusy = true;
    const next = !reposted;
    try {
      if (next) await api.repostTrack(track.id);
      else await api.unrepostTrack(track.id);
      reposted = next;
    } catch (e) {
      commentsError = `Failed to ${next ? "repost" : "un-repost"} track: ${e}`;
    }
    repostBusy = false;
  }

  async function submitComment() {
    const body = newComment.trim();
    if (!body || postingComment) return;
    postingComment = true;
    try {
      const c = await api.postComment(track.id, body);
      comments = [c, ...comments];
      newComment = "";
    } catch (e) {
      commentsError = `Failed to post comment: ${e}`;
    }
    postingComment = false;
  }

  function openArtist() {
    if (track.user) onOpenProfile(track.user.id);
  }

  function togglePlay(e: MouseEvent) {
    e.stopPropagation();
    if (!playable) return;
    if (isCurrent) player.toggle();
    else player.play(track);
  }
</script>

<button class="back" onclick={onBack}><Icon name="arrow-left" size={14} /> Back</button>

<div class="hero">
  <button class="artwork-wrap" onclick={togglePlay} aria-label={isPlaying ? "Pause" : "Play"}>
    {#if track.artwork_url}
      <img src={track.artwork_url} alt="" class="artwork" />
    {:else}
      <span class="artwork artwork-fallback"><Icon name="music" size={36} /></span>
    {/if}
    <span class="play-overlay" class:visible={isCurrent || !playable}>
      <span class="play-circle" class:play-icon={!isCurrent || !isPlaying}>
        <Icon name={!playable ? "lock" : isPlaying ? "pause" : "play"} size={24} />
      </span>
    </span>
  </button>

  <div class="meta">
    <h1>{track.title ?? `Untitled #${track.id}`}</h1>
    <div class="artist-row">
      <button class="avatar-btn" onclick={openArtist} aria-label={track.user?.username ?? "Artist"}>
        {#if track.user?.avatar_url}
          <img src={track.user.avatar_url} alt="" class="avatar" loading="lazy" />
        {:else}
          <span class="avatar avatar-fallback"><Icon name="user" size={15} /></span>
        {/if}
      </button>
      <div class="artist-meta">
        <button class="artist" onclick={openArtist}>{track.user?.username ?? "Unknown artist"}</button>
        {#if track.user && handleOf(track.user)}<span class="handle">@{handleOf(track.user)}</span>{/if}
      </div>
      {#if track.user}
        <FollowButton userId={track.user.id} permalinkUrl={track.user.permalink_url} compact />
      {/if}
    </div>
    <span class="duration">{formatDuration(track.duration)}</span>
  </div>
</div>

{#if !playable}
  <div class="drm-notice">
    <Icon name="lock" size={15} />
    <span>This track is DRM-protected (licensed major-label content) and can't be played in this app.</span>
    {#if track.permalink_url}
      <button class="open-external" onclick={() => openUrl(track.permalink_url!)}>
        Open on soundcloud.com <Icon name="external-link" size={12} />
      </button>
    {/if}
  </div>
{/if}

<div class="actions">
  <button class="play-btn" onclick={togglePlay} disabled={!playable}>
    <Icon name={isPlaying ? "pause" : "play"} size={14} /> {isPlaying ? "Pause" : "Play"}
  </button>
  <button class="action" class:active={isLiked} onclick={toggleLike} disabled={likes.isBusy(track.id)}>
    <Icon name={isLiked ? "heart-filled" : "heart"} size={14} /> {track.likes_count ?? 0}
  </button>
  <button class="action" class:active={reposted} onclick={toggleRepost} disabled={repostBusy}>
    <Icon name="repost" size={14} /> {track.reposts_count ?? 0}
  </button>
  {#if me}
    <button class="action" onclick={() => (showAddToPlaylist = true)} aria-label="Add to playlist" title="Add to playlist">
      <Icon name="plus" size={14} />
    </button>
  {/if}
  <span class="stat"><Icon name="comment" size={14} /> {track.comment_count ?? comments.length}</span>
  {#if track.playback_count}<span class="stat"><Icon name="playback" size={12} /> {track.playback_count.toLocaleString()}</span>{/if}
  <span class="spacer"></span>
  <ShareButton url={track.permalink_url} />
</div>

{#if track.user && (moreByArtistLoading || moreByArtist.length > 0)}
  <section class="shelf-section">
    <h2>More by {track.user.username ?? "this artist"}</h2>
    {#if moreByArtistLoading}
      <p class="muted">Loading...</p>
    {:else}
      <div class="shelf">
        {#each moreByArtist as t (t.id)}
          <TrackCard track={t} queue={moreByArtist} {onOpenProfile} {onOpenTrack} {me} />
        {/each}
      </div>
    {/if}
  </section>
{/if}

{#if relatedLoading || relatedTracks.length > 0}
  <section class="shelf-section">
    <h2>Related tracks</h2>
    {#if relatedLoading}
      <p class="muted">Loading...</p>
    {:else}
      <div class="shelf">
        {#each relatedTracks as t (t.id)}
          <TrackCard track={t} queue={relatedTracks} {onOpenProfile} {onOpenTrack} {me} />
        {/each}
      </div>
    {/if}
  </section>
{/if}

{#if commentsError}
  <p class="error-text">{commentsError}</p>
{/if}

<form class="comment-form" onsubmit={(e) => { e.preventDefault(); submitComment(); }}>
  {#if me?.avatar_url}
    <img src={me.avatar_url} alt="" class="comment-form-avatar" loading="lazy" />
  {:else}
    <span class="comment-form-avatar avatar-fallback"><Icon name="user" size={13} /></span>
  {/if}
  <input placeholder="Write a comment..." bind:value={newComment} disabled={postingComment} />
  {#if newComment.trim()}
    <button type="submit" disabled={postingComment}>Post</button>
  {/if}
</form>

<div class="comments">
  {#if commentsLoading}
    <p class="muted">Loading comments...</p>
  {:else if comments.length === 0}
    <p class="muted">No comments yet.</p>
  {:else}
    {#each comments as c}
      <div class="comment">
        {#if c.user?.avatar_url}
          <img src={c.user.avatar_url} alt="" class="comment-avatar" loading="lazy" />
        {:else}
          <div class="comment-avatar avatar-fallback">{(c.user?.username ?? "?")[0]?.toUpperCase()}</div>
        {/if}
        <div class="comment-body">
          <span class="comment-user">{c.user?.username ?? "Unknown"}</span>
          <span class="comment-text">{c.body}</span>
        </div>
      </div>
    {/each}
  {/if}
</div>

{#if showAddToPlaylist}
  <AddToPlaylistModal {track} {me} onClose={() => (showAddToPlaylist = false)} />
{/if}

<style>
.back {
  display: flex;
  align-items: center;
  gap: 0.3rem;
  background: none;
  border: none;
  color: var(--muted);
  cursor: pointer;
  padding: 0;
  margin-bottom: 1.25rem;
  font: inherit;
}

.back:hover {
  color: var(--fg);
}

.hero {
  display: flex;
  gap: 1.5rem;
  margin-bottom: 1.5rem;
}

.artwork-wrap {
  position: relative;
  width: 168px;
  height: 168px;
  flex-shrink: 0;
  border-radius: 14px;
  overflow: hidden;
  background: var(--artwork-bg);
  border: none;
  padding: 0;
  cursor: pointer;
  box-shadow: 0 12px 28px rgba(0, 0, 0, 0.22);
}

.artwork {
  width: 100%;
  height: 100%;
  object-fit: cover;
  display: block;
}

.artwork-fallback {
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--muted);
}

.play-overlay {
  position: absolute;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.3);
  opacity: 0;
  transition: opacity 0.18s ease;
}

.artwork-wrap:hover .play-overlay,
.play-overlay.visible {
  opacity: 1;
}

.play-circle {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 58px;
  height: 58px;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.92);
  color: #111;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.3);
}

/* Optical centering for the play triangle only -- same nudge as
   FeedPost.svelte/PlaylistCard.svelte's identical fix. */
.play-circle.play-icon {
  padding-right: 3px;
}

.meta {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  min-width: 0;
  justify-content: center;
}

.meta h1 {
  margin: 0;
  font-size: 1.5rem;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.artist-row {
  display: flex;
  align-items: center;
  gap: 0.6rem;
  min-width: 0;
}

.avatar-btn {
  background: none;
  border: none;
  padding: 0;
  cursor: pointer;
  flex-shrink: 0;
}

.avatar {
  width: 34px;
  height: 34px;
  border-radius: 50%;
  object-fit: cover;
  background: var(--artwork-bg);
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--muted);
}

.artist-meta {
  display: flex;
  flex-direction: column;
  min-width: 0;
}

.artist {
  background: none;
  border: none;
  padding: 0;
  color: var(--fg);
  cursor: pointer;
  font: inherit;
  font-weight: 700;
  font-size: 0.95rem;
  text-align: left;
  width: fit-content;
  max-width: 100%;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.artist:hover {
  text-decoration: underline;
}

.handle {
  color: var(--muted);
  font-size: 0.78rem;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.duration {
  color: var(--muted);
  font-size: 0.82rem;
  font-variant-numeric: tabular-nums;
}

.actions {
  display: flex;
  align-items: center;
  gap: 0.6rem;
  margin-bottom: 1.25rem;
  flex-wrap: wrap;
}

.play-btn {
  display: flex;
  align-items: center;
  gap: 0.4rem;
  background: var(--accent);
  color: white;
  border: none;
  border-radius: 999px;
  padding: 0.5rem 1.3rem;
  font-weight: 600;
  cursor: pointer;
}

.play-btn:hover:not(:disabled) {
  background: var(--accent-hover);
}

.play-btn:disabled {
  background: var(--border);
  color: var(--muted);
  cursor: default;
}

.drm-notice {
  display: flex;
  align-items: center;
  gap: 0.6rem;
  flex-wrap: wrap;
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 12px;
  padding: 0.7rem 0.9rem;
  margin-bottom: 1.25rem;
  font-size: 0.85rem;
  color: var(--muted);
}

.open-external {
  display: flex;
  align-items: center;
  gap: 0.3rem;
  background: none;
  border: 1px solid var(--border);
  border-radius: 999px;
  padding: 0.3rem 0.7rem;
  color: var(--fg);
  cursor: pointer;
  font: inherit;
  font-size: 0.8rem;
  margin-left: auto;
}

.open-external:hover {
  border-color: var(--accent);
  color: var(--accent);
}

.action {
  display: flex;
  align-items: center;
  gap: 0.35rem;
  background: none;
  border: 1px solid var(--border);
  border-radius: 999px;
  padding: 0.45rem 0.9rem;
  cursor: pointer;
  color: var(--muted);
  font: inherit;
  font-size: 0.85rem;
  font-variant-numeric: tabular-nums;
}

.action:hover:not(:disabled) {
  border-color: var(--accent);
  color: var(--accent);
}

.action.active {
  border-color: var(--accent);
  color: var(--accent);
}

.action:disabled {
  opacity: 0.6;
  cursor: default;
}

.stat {
  display: inline-flex;
  align-items: center;
  gap: 0.3rem;
  color: var(--muted);
  font-size: 0.85rem;
}

.spacer {
  flex: 1;
}

.error-text {
  color: var(--error-text);
  font-size: 0.85rem;
  margin-bottom: 0.75rem;
}

.comment-form {
  display: flex;
  align-items: center;
  gap: 0.6rem;
  margin-bottom: 1.5rem;
  max-width: 36rem;
}

.comment-form-avatar {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  object-fit: cover;
  flex-shrink: 0;
  background: var(--artwork-bg);
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--muted);
}

.comment-form input {
  flex: 1;
  min-width: 0;
  padding: 0.6em 1em;
  border-radius: 999px;
  border: 1px solid var(--border);
  background: var(--surface);
  color: inherit;
  font: inherit;
}

.comment-form button {
  flex-shrink: 0;
  padding: 0.55em 1.1em;
  border-radius: 999px;
  border: none;
  background: var(--accent);
  color: white;
  cursor: pointer;
  font: inherit;
  font-weight: 600;
}

.comment-form button:hover:not(:disabled) {
  background: var(--accent-hover);
}

.comment-form button:disabled {
  opacity: 0.5;
  cursor: default;
}

.comments {
  display: flex;
  flex-direction: column;
  gap: 0.9rem;
  max-width: 36rem;
  padding-bottom: 2rem;
}

.muted {
  color: var(--muted);
}

.comment {
  display: flex;
  gap: 0.6rem;
}

.comment-avatar {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  object-fit: cover;
  flex-shrink: 0;
  background: var(--artwork-bg);
}

.avatar-fallback {
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 0.8rem;
  color: var(--muted);
}

.comment-body {
  display: flex;
  flex-direction: column;
  font-size: 0.88rem;
  min-width: 0;
  padding: 0.5rem 0.75rem;
  background: var(--surface);
  border-radius: 12px;
}

.comment-user {
  font-weight: 700;
  font-size: 0.82rem;
}

.comment-text {
  overflow-wrap: break-word;
}

.shelf-section {
  margin-bottom: 2rem;
}

.shelf-section h2 {
  margin: 0 0 0.85rem;
  font-size: 1.05rem;
}

.shelf {
  display: flex;
  gap: 1.1rem;
  overflow-x: auto;
  scroll-behavior: smooth;
  padding: 0.1rem 0.1rem 0.5rem;
  /* Same reasoning as PlaylistShelf.svelte's identical rule -- a visible
     scrollbar under a horizontal card rail reads as unfinished. */
  scrollbar-width: none;
}

.shelf::-webkit-scrollbar {
  display: none;
}

.shelf > :global(.track-tile) {
  flex: 0 0 152px;
}
</style>
