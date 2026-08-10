<script lang="ts">
  import { isSystemPlaylist, selectionArtwork, type Playlist, type SystemPlaylist } from "../types";
  import Icon from "./Icon.svelte";

  let { item, onOpen }: { item: Playlist | SystemPlaylist; onOpen: (item: Playlist | SystemPlaylist) => void } = $props();

  let artwork = $derived(selectionArtwork(item));
  let isMix = $derived(isSystemPlaylist(item));
  let trackCount = $derived(isSystemPlaylist(item) ? item.tracks.length : item.track_count);
  let meta = $derived(isMix ? "Mix" : trackCount != null ? `Playlist · ${trackCount} track${trackCount === 1 ? "" : "s"}` : "Playlist");
</script>

<button class="playlist-row-item" onclick={() => onOpen(item)}>
  {#if artwork}
    <img src={artwork} alt="" class="artwork" loading="lazy" />
  {:else}
    <span class="artwork artwork-fallback"><Icon name="queue" size={16} /></span>
  {/if}
  <span class="info">
    <span class="row-title">{item.title ?? "Untitled"}</span>
    <span class="row-meta">{meta}</span>
  </span>
</button>

<style>
.playlist-row-item {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  width: 100%;
  background: none;
  border: none;
  padding: 0.4rem 0.5rem;
  border-radius: 6px;
  text-align: left;
  color: inherit;
  font: inherit;
  cursor: pointer;
  transition: background-color 0.12s ease;
}

.playlist-row-item:hover {
  background: var(--row-hover);
}

.artwork {
  width: 44px;
  height: 44px;
  flex-shrink: 0;
  border-radius: 5px;
  object-fit: cover;
  background: var(--artwork-bg);
}

.artwork-fallback {
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--muted);
}

.info {
  display: flex;
  flex-direction: column;
  min-width: 0;
}

.row-title {
  font-weight: 600;
  font-size: 0.9rem;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.row-meta {
  font-size: 0.78rem;
  color: var(--muted);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>
