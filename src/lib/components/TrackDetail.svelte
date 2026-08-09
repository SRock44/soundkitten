<script lang="ts">
  import { api } from "../api";
  import { player } from "../stores/player.svelte";
  import { likes } from "../stores/likes.svelte";
  import { formatDuration, handleOf, isPlayable } from "../types";
  import type { Comment, Track } from "../types";
  import Icon from "./Icon.svelte";
  import ShareButton from "./ShareButton.svelte";
  import { openUrl } from "@tauri-apps/plugin-opener";

  let { track, onBack, onOpenProfile }: { track: Track; onBack: () => void; onOpenProfile: (id: number) => void } = $props();

  let isLiked = $derived(likes.has(track.id));
  let isCurrent = $derived(player.current?.id === track.id);
  let isPlaying = $derived(isCurrent && player.isPlaying);
  let playable = $derived(isPlayable(track));

  let reposted = $state(false);
  let likeBusy = $state(false);
  let repostBusy = $state(false);

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

  async function toggleLike() {
    if (likeBusy) return;
    likeBusy = true;
    const next = !isLiked;
    try {
      if (next) await api.likeTrack(track.id);
      else await api.unlikeTrack(track.id);
      likes.set(track.id, next);
    } catch (e) {
      commentsError = `Failed to ${next ? "like" : "unlike"} track: ${e}`;
    }
    likeBusy = false;
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
</script>

<button class="back" onclick={onBack}><Icon name="arrow-left" size={14} /> Back</button>

<div class="header">
  {#if track.artwork_url}
    <img src={track.artwork_url} alt="" class="artwork" />
  {:else}
    <div class="artwork artwork-fallback"><Icon name="music" size={28} /></div>
  {/if}
  <div class="meta">
    <h1>{track.title ?? `Untitled #${track.id}`}</h1>
    <button class="artist" onclick={openArtist}>{track.user?.username ?? "Unknown artist"}</button>
    {#if track.user && handleOf(track.user)}<span class="handle">@{handleOf(track.user)}</span>{/if}
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
  <button class="play-btn" onclick={() => player.play(track)} disabled={!playable}>
    <Icon name={isPlaying ? "pause" : "play"} size={14} /> {isPlaying ? "Pause" : "Play"}
  </button>
  <button class="action" class:active={isLiked} onclick={toggleLike} disabled={likeBusy}>
    <Icon name={isLiked ? "heart-filled" : "heart"} size={14} /> {track.likes_count ?? 0}
  </button>
  <button class="action" class:active={reposted} onclick={toggleRepost} disabled={repostBusy}>
    <Icon name="repost" size={14} /> {track.reposts_count ?? 0}
  </button>
  <span class="stat"><Icon name="comment" size={14} /> {track.comment_count ?? comments.length}</span>
  {#if track.playback_count}<span class="stat"><Icon name="playback" size={12} /> {track.playback_count.toLocaleString()}</span>{/if}
  <ShareButton url={track.permalink_url} />
</div>

{#if commentsError}
  <p class="error-text">{commentsError}</p>
{/if}

<form class="comment-form" onsubmit={(e) => { e.preventDefault(); submitComment(); }}>
  <input placeholder="Write a comment..." bind:value={newComment} disabled={postingComment} />
  <button type="submit" disabled={postingComment || !newComment.trim()}>Post</button>
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
          <img src={c.user.avatar_url} alt="" class="comment-avatar" />
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
  margin-bottom: 1rem;
  font: inherit;
}

.header {
  display: flex;
  gap: 1.25rem;
  margin-bottom: 1.25rem;
}

.artwork {
  width: 130px;
  height: 130px;
  border-radius: 8px;
  object-fit: cover;
  flex-shrink: 0;
  background: var(--artwork-bg);
}

.artwork-fallback {
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--muted);
  font-size: 2.4rem;
}

.meta {
  display: flex;
  flex-direction: column;
  gap: 0.2rem;
  min-width: 0;
  justify-content: center;
}

.meta h1 {
  margin: 0;
  font-size: 1.3rem;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.artist {
  background: none;
  border: none;
  padding: 0;
  color: var(--fg);
  cursor: pointer;
  font: inherit;
  font-weight: 600;
  text-align: left;
  width: fit-content;
}

.artist:hover {
  text-decoration: underline;
}

.handle,
.duration {
  color: var(--muted);
  font-size: 0.82rem;
}

.actions {
  display: flex;
  align-items: center;
  gap: 0.6rem;
  margin-bottom: 1rem;
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
  background: var(--row-hover);
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 0.7rem 0.9rem;
  margin-bottom: 1rem;
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
  color: inherit;
  font: inherit;
  font-size: 0.85rem;
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

.error-text {
  color: var(--error-text);
  font-size: 0.85rem;
  margin-bottom: 0.75rem;
}

.comment-form {
  display: flex;
  gap: 0.5rem;
  margin-bottom: 1.25rem;
  max-width: 36rem;
}

.comment-form input {
  flex: 1;
  padding: 0.5em 0.75em;
  border-radius: 6px;
  border: 1px solid var(--border);
  background: var(--nav-bg);
  color: inherit;
}

.comment-form button {
  padding: 0.5em 1em;
  border-radius: 6px;
  border: none;
  background: var(--accent);
  color: white;
  cursor: pointer;
}

.comment-form button:disabled {
  opacity: 0.5;
  cursor: default;
}

.comments {
  display: flex;
  flex-direction: column;
  gap: 0.85rem;
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
  width: 30px;
  height: 30px;
  border-radius: 50%;
  object-fit: cover;
  flex-shrink: 0;
  background: var(--artwork-bg);
}

.avatar-fallback {
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 0.78rem;
  color: var(--muted);
}

.comment-body {
  display: flex;
  flex-direction: column;
  font-size: 0.88rem;
  min-width: 0;
}

.comment-user {
  font-weight: 600;
  font-size: 0.82rem;
}

.comment-text {
  overflow-wrap: break-word;
}
</style>
