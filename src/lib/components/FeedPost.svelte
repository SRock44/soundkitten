<script lang="ts">
  import { openUrl } from "@tauri-apps/plugin-opener";
  import { api } from "../api";
  import { player } from "../stores/player.svelte";
  import { likes } from "../stores/likes.svelte";
  import { officialAuth } from "../stores/officialAuth.svelte";
  import type { Comment, FeedEntry, Profile, Track } from "../types";
  import { formatDuration, handleOf, isPlayable, timeAgo } from "../types";
  import Icon from "./Icon.svelte";
  import FollowButton from "./FollowButton.svelte";
  import ShareButton from "./ShareButton.svelte";
  import AddToPlaylistModal from "./AddToPlaylistModal.svelte";

  let {
    entry,
    queue,
    onOpenProfile,
    onOpenTrack,
    me = null,
  }: {
    entry: FeedEntry;
    queue: Track[];
    onOpenProfile?: (id: number) => void;
    onOpenTrack?: (t: Track) => void;
    me?: Profile | null;
  } = $props();

  let track = $derived(entry.track);
  let isCurrent = $derived(player.current?.id === track.id);
  let isPlaying = $derived(isCurrent && player.isPlaying);
  let playable = $derived(isPlayable(track));
  let isLiked = $derived(likes.has(track.id));

  function togglePlay(e: MouseEvent) {
    e.stopPropagation();
    if (!playable) return;
    if (isCurrent) player.toggle();
    else player.play(track, queue);
  }

  async function toggleLike() {
    try {
      if ((await likes.toggle(track)) === "declined" && track.permalink_url) openUrl(track.permalink_url);
    } catch (e) {
      likeError = `Failed to ${isLiked ? "unlike" : "like"}: ${e}`;
    }
  }

  let likeError = $state("");

  // Reposting has no read-back -- the API never tells us whether *we*
  // already reposted a track, only the count -- so this starts unset
  // like TrackDetail.svelte's own repost toggle does, a pre-existing app
  // limitation, not something new here.
  let reposted = $state(false);
  let repostBusy = $state(false);

  async function toggleRepost() {
    if (repostBusy) return;
    repostBusy = true;
    const next = !reposted;
    try {
      if (next) await api.repostTrack(track.id);
      else await api.unrepostTrack(track.id);
      reposted = next;
    } catch (e) {
      likeError = `Failed to ${next ? "repost" : "un-repost"}: ${e}`;
    }
    repostBusy = false;
  }

  let showComments = $state(false);
  let comments = $state<Comment[] | null>(null);
  let commentsLoading = $state(false);
  let commentsError = $state("");
  let newComment = $state("");
  let postingComment = $state(false);

  // Lazy: only fetched the first time a card's comments are actually
  // opened, not for every post in the feed up front -- fetching comments
  // for 15-50 feed items on load would be exactly the kind of request
  // burst that's gotten this app's account rate-limited before.
  async function toggleComments() {
    showComments = !showComments;
    if (showComments && comments === null) {
      commentsLoading = true;
      commentsError = "";
      try {
        comments = await api.trackComments(track.id);
      } catch (e) {
        commentsError = `Failed to load comments: ${e}`;
      }
      commentsLoading = false;
    }
  }

  async function submitComment() {
    const body = newComment.trim();
    if (!body || postingComment) return;
    postingComment = true;
    try {
      const c = await api.postComment(track.id, body);
      comments = [c, ...(comments ?? [])];
      newComment = "";
    } catch (e) {
      commentsError = `Failed to post comment: ${e}`;
    }
    postingComment = false;
  }

  let showAddToPlaylist = $state(false);

  function openReposter() {
    if (entry.reposted_by && onOpenProfile) onOpenProfile(entry.reposted_by.id);
  }

  function openArtist() {
    if (track.user && onOpenProfile) onOpenProfile(track.user.id);
  }

  function openTrackPage() {
    onOpenTrack?.(track);
  }
</script>

<article class="post">
  {#if entry.is_repost && entry.reposted_by}
    <button class="repost-tag" onclick={openReposter}>
      <Icon name="repost" size={13} />
      <span>Reposted by <strong>{entry.reposted_by.username ?? "someone"}</strong></span>
    </button>
  {/if}

  <div class="post-header">
    <button class="avatar-btn" onclick={openArtist} aria-label={track.user?.username ?? "Artist"}>
      {#if track.user?.avatar_url}
        <img src={track.user.avatar_url} alt="" class="avatar" loading="lazy" />
      {:else}
        <span class="avatar avatar-fallback"><Icon name="user" size={16} /></span>
      {/if}
    </button>
    <div class="header-meta">
      <button class="username" onclick={openArtist}>{track.user?.username ?? "Unknown artist"}</button>
      <span class="header-sub">
        {#if track.user && handleOf(track.user)}@{handleOf(track.user)}{/if}
        {#if entry.activity_at}<span class="dot">·</span>{timeAgo(entry.activity_at)}{/if}
      </span>
    </div>
    {#if track.user}
      <FollowButton userId={track.user.id} permalinkUrl={track.user.permalink_url} compact />
    {/if}
  </div>

  <button class="artwork-wrap" onclick={togglePlay} aria-label={isPlaying ? "Pause" : "Play"}>
    {#if track.artwork_url}
      <img src={track.artwork_url} alt="" class="artwork" loading="lazy" />
    {:else}
      <span class="artwork artwork-fallback"><Icon name="music" size={40} /></span>
    {/if}
    <span class="play-overlay" class:visible={isCurrent || !playable}>
      <span class="play-btn">
        <Icon name={!playable ? "lock" : isPlaying ? "pause" : "play"} size={22} />
      </span>
    </span>
  </button>

  <div class="post-body">
    <button class="title" onclick={openTrackPage}>{track.title ?? `Untitled #${track.id}`}</button>
    <span class="duration">{formatDuration(track.duration)}</span>
  </div>

  {#if likeError}<p class="error-text">{likeError}</p>{/if}

  <div class="action-bar">
    <button class="action" class:active={isLiked} onclick={toggleLike} disabled={likes.isBusy(track.id)}>
      <Icon name={isLiked ? "heart-filled" : "heart"} size={16} /> {track.likes_count ?? 0}
    </button>
    <button class="action" class:active={showComments} onclick={toggleComments}>
      <Icon name="comment" size={16} /> {track.comment_count ?? 0}
    </button>
    <button class="action" class:active={reposted} onclick={toggleRepost} disabled={repostBusy}>
      <Icon name="repost" size={16} /> {track.reposts_count ?? 0}
    </button>
    <button class="action" onclick={() => (showAddToPlaylist = true)} aria-label="Add to playlist" title="Add to playlist">
      <Icon name="plus" size={16} />
    </button>
    <span class="spacer"></span>
    <ShareButton url={track.permalink_url} label="" />
  </div>

  {#if showComments}
    <div class="comments">
      {#if !officialAuth.connected}
        <p class="comment-hint">Comments are read-only until you connect the official login (like/reply prompts you the first time you try).</p>
      {/if}
      <form class="comment-form" onsubmit={(e) => { e.preventDefault(); submitComment(); }}>
        <input placeholder="Write a comment..." bind:value={newComment} disabled={postingComment} />
        <button type="submit" disabled={postingComment || !newComment.trim()}>Post</button>
      </form>
      {#if commentsLoading}
        <p class="muted">Loading comments...</p>
      {:else if commentsError}
        <p class="error-text">{commentsError}</p>
      {:else if comments && comments.length === 0}
        <p class="muted">No comments yet.</p>
      {:else if comments}
        <div class="comment-list">
          {#each comments as c}
            <div class="comment">
              {#if c.user?.avatar_url}
                <img src={c.user.avatar_url} alt="" class="comment-avatar" loading="lazy" />
              {:else}
                <div class="comment-avatar avatar-fallback"><Icon name="user" size={12} /></div>
              {/if}
              <div class="comment-body">
                <span class="comment-user">{c.user?.username ?? "Unknown"}</span>
                <span class="comment-text">{c.body}</span>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  {/if}
</article>

{#if showAddToPlaylist}
  <AddToPlaylistModal {track} {me} onClose={() => (showAddToPlaylist = false)} />
{/if}

<style>
.post {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  background: var(--surface);
  border-radius: 16px;
  padding: 1.1rem;
}

.repost-tag {
  display: flex;
  align-items: center;
  gap: 0.4rem;
  background: none;
  border: none;
  padding: 0;
  margin: 0 0 -0.25rem;
  color: var(--muted);
  font: inherit;
  font-size: 0.78rem;
  cursor: pointer;
  width: fit-content;
}

.repost-tag:hover {
  color: var(--fg);
}

.repost-tag strong {
  font-weight: 600;
  color: inherit;
}

.post-header {
  display: flex;
  align-items: center;
  gap: 0.65rem;
}

.avatar-btn {
  background: none;
  border: none;
  padding: 0;
  cursor: pointer;
  flex-shrink: 0;
}

.avatar {
  width: 38px;
  height: 38px;
  border-radius: 50%;
  object-fit: cover;
  background: var(--artwork-bg);
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--muted);
}

.header-meta {
  display: flex;
  flex-direction: column;
  min-width: 0;
  flex: 1;
}

.username {
  background: none;
  border: none;
  padding: 0;
  color: var(--fg);
  font-weight: 700;
  font-size: 0.92rem;
  cursor: pointer;
  text-align: left;
  width: fit-content;
  max-width: 100%;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.username:hover {
  text-decoration: underline;
}

.header-sub {
  font-size: 0.78rem;
  color: var(--muted);
}

.dot {
  margin: 0 0.3rem;
}

.artwork-wrap {
  position: relative;
  display: block;
  width: 100%;
  aspect-ratio: 16 / 9;
  border-radius: 12px;
  overflow: hidden;
  background: var(--artwork-bg);
  border: none;
  padding: 0;
  cursor: pointer;
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
  background: rgba(0, 0, 0, 0.25);
  opacity: 0;
  transition: opacity 0.15s ease;
}

.artwork-wrap:hover .play-overlay,
.play-overlay.visible {
  opacity: 1;
}

.play-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 52px;
  height: 52px;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.92);
  color: #111;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.3);
}

.post-body {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: 0.75rem;
}

.title {
  background: none;
  border: none;
  padding: 0;
  color: var(--fg);
  font-weight: 700;
  font-size: 1.05rem;
  cursor: pointer;
  text-align: left;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  min-width: 0;
}

.title:hover {
  text-decoration: underline;
}

.duration {
  flex-shrink: 0;
  color: var(--muted);
  font-size: 0.82rem;
  font-variant-numeric: tabular-nums;
}

.action-bar {
  display: flex;
  align-items: center;
  gap: 0.4rem;
}

.action {
  display: flex;
  align-items: center;
  gap: 0.35rem;
  background: none;
  border: 1px solid var(--border);
  border-radius: 999px;
  padding: 0.4rem 0.75rem;
  color: var(--muted);
  cursor: pointer;
  font: inherit;
  font-size: 0.82rem;
  font-variant-numeric: tabular-nums;
}

.action:hover {
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

.spacer {
  flex: 1;
}

.error-text {
  color: var(--error-text);
  font-size: 0.82rem;
  margin: 0;
}

.comments {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  padding-top: 0.5rem;
  border-top: 1px solid var(--border);
}

.comment-hint {
  margin: 0;
  font-size: 0.78rem;
  color: var(--muted);
}

.comment-form {
  display: flex;
  gap: 0.5rem;
}

.comment-form input {
  flex: 1;
  padding: 0.5em 0.75em;
  border-radius: 6px;
  border: 1px solid var(--border);
  background: var(--nav-bg);
  color: inherit;
  font: inherit;
  font-size: 0.85rem;
}

.comment-form button {
  padding: 0.5em 1em;
  border-radius: 6px;
  border: none;
  background: var(--accent);
  color: white;
  cursor: pointer;
  font: inherit;
  font-size: 0.85rem;
}

.comment-form button:disabled {
  opacity: 0.5;
  cursor: default;
}

.muted {
  color: var(--muted);
  font-size: 0.85rem;
  margin: 0;
}

.comment-list {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  max-height: 16rem;
  overflow-y: auto;
}

.comment {
  display: flex;
  gap: 0.55rem;
}

.comment-avatar {
  width: 26px;
  height: 26px;
  border-radius: 50%;
  object-fit: cover;
  flex-shrink: 0;
  background: var(--artwork-bg);
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--muted);
}

.comment-body {
  display: flex;
  flex-direction: column;
  font-size: 0.85rem;
  min-width: 0;
}

.comment-user {
  font-weight: 600;
  font-size: 0.8rem;
}

.comment-text {
  overflow-wrap: break-word;
}
</style>
