<script lang="ts">
  import type { FeedEntry, Profile, Track } from "../types";
  import { timeAgo } from "../types";
  import TrackRow from "./TrackRow.svelte";
  import Icon from "./Icon.svelte";

  let {
    entry,
    index,
    queue,
    onOpenProfile,
    onOpenTrack,
    me = null,
  }: {
    entry: FeedEntry;
    index?: number;
    queue: Track[];
    onOpenProfile?: (id: number) => void;
    onOpenTrack?: (t: Track) => void;
    me?: Profile | null;
  } = $props();

  function openReposter() {
    if (entry.reposted_by && onOpenProfile) onOpenProfile(entry.reposted_by.id);
  }
</script>

<div class="feed-item">
  {#if entry.is_repost && entry.reposted_by}
    <button class="origin" onclick={openReposter}>
      {#if entry.reposted_by.avatar_url}
        <img src={entry.reposted_by.avatar_url} alt="" class="origin-avatar" loading="lazy" />
      {:else}
        <span class="origin-avatar origin-avatar-fallback"><Icon name="repost" size={9} /></span>
      {/if}
      <span class="origin-text">Reposted by <strong>{entry.reposted_by.username ?? "someone"}</strong></span>
      {#if entry.activity_at}<span class="origin-time">{timeAgo(entry.activity_at)}</span>{/if}
    </button>
  {/if}
  <TrackRow track={entry.track} {queue} {index} {onOpenProfile} {onOpenTrack} {me} />
</div>

<style>
.feed-item {
  border-radius: 8px;
}

.origin {
  display: flex;
  align-items: center;
  gap: 0.4rem;
  background: none;
  border: none;
  padding: 0.5rem 0.6rem 0.15rem;
  margin: 0;
  color: var(--muted);
  font: inherit;
  font-size: 0.76rem;
  cursor: pointer;
  width: fit-content;
  max-width: 100%;
}

.origin:hover .origin-text {
  color: var(--fg);
}

.origin-avatar {
  width: 15px;
  height: 15px;
  border-radius: 50%;
  object-fit: cover;
  flex-shrink: 0;
  background: var(--artwork-bg);
}

.origin-avatar-fallback {
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--muted);
}

.origin-text {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.origin-text strong {
  font-weight: 600;
  color: inherit;
}

.origin-time {
  flex-shrink: 0;
  color: var(--muted);
  opacity: 0.75;
}

.origin-time::before {
  content: "·";
  margin-right: 0.4rem;
}
</style>
