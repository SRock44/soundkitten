<script lang="ts">
  import { player } from "../stores/player.svelte";
  import { formatDuration, isPlayable } from "../types";
  import type { Track } from "../types";
  import Icon from "./Icon.svelte";

  let {
    track,
    queue = [],
    onOpenProfile,
    onOpenTrack,
  }: {
    track: Track;
    queue?: Track[];
    onOpenProfile?: (id: number) => void;
    onOpenTrack?: (t: Track) => void;
  } = $props();

  let isCurrent = $derived(player.current?.id === track.id);
  let isPlaying = $derived(isCurrent && player.isPlaying);
  let playable = $derived(isPlayable(track));

  function togglePlay(e: MouseEvent) {
    e.stopPropagation();
    if (!playable) return;
    if (isCurrent) player.toggle();
    else player.play(track, queue);
  }

  function openTrack() {
    onOpenTrack?.(track);
  }

  function openArtist(e: MouseEvent) {
    if (!track.user || !onOpenProfile) return;
    e.stopPropagation();
    onOpenProfile(track.user.id);
  }
</script>

<div class="track-tile">
  <span class="card-surface">
    <button class="artwork-wrap" onclick={togglePlay} aria-label={isPlaying ? "Pause" : "Play"}>
      {#if track.artwork_url}
        <img src={track.artwork_url} alt="" loading="lazy" />
      {:else}
        <span class="artwork-fallback"><Icon name="music" size={22} /></span>
      {/if}
      <span class="play-overlay" class:visible={isCurrent || !playable} class:play-icon={!isCurrent || !isPlaying}>
        <Icon name={!playable ? "lock" : isPlaying ? "pause" : "play"} size={17} />
      </span>
    </button>
    <span class="card-text">
      {#if onOpenTrack}
        <button class="tile-title tile-title-btn" onclick={openTrack}>{track.title ?? `Untitled #${track.id}`}</button>
      {:else}
        <span class="tile-title">{track.title ?? `Untitled #${track.id}`}</span>
      {/if}
      <span class="tile-meta-row">
        <button class="tile-artist" onclick={openArtist}>{track.user?.username ?? "Unknown artist"}</button>
        <span class="tile-duration">{formatDuration(track.duration)}</span>
      </span>
    </span>
  </span>
</div>

<style>
.track-tile {
  display: flex;
  flex-direction: column;
  flex: 0 0 172px;
  min-width: 0;
  width: 100%;
}

.card-surface {
  display: flex;
  flex-direction: column;
  gap: 0.85rem;
  padding: 0.85rem;
  border-radius: 14px;
  background: var(--surface);
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.06);
  transition: background-color 0.15s ease, transform 0.15s ease, box-shadow 0.15s ease;
}

.track-tile:hover .card-surface {
  background: var(--surface-hover);
  transform: translateY(-4px);
  box-shadow: 0 18px 32px rgba(0, 0, 0, 0.22);
}

.artwork-wrap {
  position: relative;
  width: 100%;
  aspect-ratio: 1;
  border-radius: 8px;
  overflow: hidden;
  background: var(--artwork-bg);
  border: none;
  padding: 0;
  cursor: pointer;
}

.artwork-wrap img,
.artwork-fallback {
  width: 100%;
  height: 100%;
  object-fit: cover;
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
  background: rgba(0, 0, 0, 0.35);
  color: white;
  opacity: 0;
  transition: opacity 0.18s ease;
}

/* Optical centering for the play triangle only -- same nudge as
   PlaylistCard.svelte's identical fix, the pause/lock glyphs don't need it. */
.play-overlay.play-icon {
  padding-right: 2.5px;
}

.track-tile:hover .play-overlay,
.play-overlay.visible {
  opacity: 1;
}

.card-text {
  display: flex;
  flex-direction: column;
  gap: 0.2rem;
  min-width: 0;
}

.tile-title {
  font-weight: 700;
  font-size: 0.92rem;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.tile-title-btn {
  background: none;
  border: none;
  padding: 0;
  color: inherit;
  font: inherit;
  font-weight: 700;
  cursor: pointer;
  text-align: left;
  width: 100%;
}

.tile-title-btn:hover {
  text-decoration: underline;
}

.tile-meta-row {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: 0.5rem;
  min-width: 0;
}

.tile-artist {
  background: none;
  border: none;
  padding: 0;
  color: var(--muted);
  font: inherit;
  font-size: 0.78rem;
  cursor: pointer;
  text-align: left;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.tile-artist:hover {
  color: var(--fg);
  text-decoration: underline;
}

.tile-duration {
  flex-shrink: 0;
  font-size: 0.78rem;
  color: var(--muted);
  font-variant-numeric: tabular-nums;
}
</style>
