<script lang="ts">
  import { api } from "../api";
  import { player } from "../stores/player.svelte";
  import { isSystemPlaylist, selectionArtwork, type Playlist, type SystemPlaylist, type Track } from "../types";
  import Icon from "./Icon.svelte";

  let { item, onOpen }: { item: Playlist | SystemPlaylist; onOpen: (item: Playlist | SystemPlaylist) => void } = $props();

  let artwork = $derived(selectionArtwork(item));
  let isMix = $derived(isSystemPlaylist(item));
  let trackCount = $derived(isSystemPlaylist(item) ? item.tracks.length : item.track_count);
  let meta = $derived(isMix ? "Mix" : trackCount != null ? `${trackCount} track${trackCount === 1 ? "" : "s"}` : "Playlist");

  let itemKey = $derived(isSystemPlaylist(item) ? `s:${item.id}` : `p:${item.id}`);
  let isActive = $derived(player.currentPlaylistKey === itemKey);
  let isActivePlaying = $derived(isActive && player.isPlaying);

  let playBusy = $state(false);

  // Listing endpoints (this card's data source, whether Home's shelves or
  // Profile's grid) only return playlist metadata -- track_count, not the
  // actual tracks (confirmed live: item.tracks is empty despite a nonzero
  // count). Starting playback needs the real track list, so this hydrates
  // it first via the same endpoints +page.svelte already uses when
  // actually opening a playlist/mix. If this card's playlist is already
  // the active queue, just toggle play/pause in place instead of
  // re-fetching and restarting it from track 1.
  async function playItem(e: MouseEvent) {
    e.stopPropagation();
    if (isActive) {
      player.toggle();
      return;
    }
    if (playBusy) return;
    playBusy = true;
    try {
      let tracks: Track[];
      if (isSystemPlaylist(item)) {
        tracks = await api.systemPlaylistTracks(item.tracks.map((t) => t.id));
      } else if (item.tracks.length > 0) {
        tracks = item.tracks;
      } else {
        tracks = (await api.playlist(item.id)).tracks;
      }
      if (tracks.length > 0) {
        player.play(tracks[0], tracks);
        player.currentPlaylistKey = itemKey;
      }
    } catch (e) {
      player.error = `Failed to play: ${e}`;
    } finally {
      playBusy = false;
    }
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
<div class="playlist-tile" role="button" tabindex="0" onclick={() => onOpen(item)}>
  <span class="card-surface">
    <span class="artwork-wrap">
      {#if artwork}
        <img src={artwork} alt="" loading="lazy" />
      {:else}
        <span class="artwork-fallback"><Icon name="queue" size={22} /></span>
      {/if}
      {#if isMix}<span class="mix-badge">Mix</span>{/if}
      <button
        class="play-overlay"
        class:visible={isActive}
        class:play-icon={!isActivePlaying}
        onclick={playItem}
        disabled={playBusy}
        aria-label={isActivePlaying ? "Pause" : "Play"}
        title={isActivePlaying ? "Pause" : "Play"}
      >
        <Icon name={isActivePlaying ? "pause" : "play"} size={17} />
      </button>
    </span>
    <span class="card-text">
      <span class="tile-title">{item.title ?? "Untitled"}</span>
      <span class="tile-meta">{meta}</span>
    </span>
  </span>
</div>

<style>
.playlist-tile {
  display: flex;
  flex-direction: column;
  /* flex-basis wins over width when this is a flex item (the horizontal
     shelves in Home.svelte via PlaylistShelf), giving a fixed-width card;
     when it's a grid item instead (Profile.svelte's wrapping
     playlist-grid), flex-* is inert and width:100% fills the grid cell. */
  flex: 0 0 172px;
  /* A flex item's default min-width is "auto" (content-based), so an
     unbreakable long title can force the card wider than its flex-basis --
     text-overflow:ellipsis on .tile-title only truncates the painted
     text, it doesn't shrink the box's computed minimum size. Same fix as
     +page.svelte's .window-body already uses for the same reason. */
  min-width: 0;
  width: 100%;
  background: none;
  border: none;
  padding: 0;
  text-align: left;
  color: inherit;
  font: inherit;
  cursor: pointer;
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

.playlist-tile:hover .card-surface {
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

.mix-badge {
  position: absolute;
  top: 8px;
  left: 8px;
  background: rgba(0, 0, 0, 0.6);
  color: #fff;
  font-size: 0.62rem;
  font-weight: 700;
  letter-spacing: 0.04em;
  text-transform: uppercase;
  padding: 0.2rem 0.5rem;
  border-radius: 999px;
}

.play-overlay {
  position: absolute;
  right: 8px;
  bottom: 8px;
  width: 38px;
  height: 38px;
  border: none;
  border-radius: 50%;
  background: var(--accent);
  color: #fff;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0;
  cursor: pointer;
  box-shadow: 0 8px 16px rgba(0, 0, 0, 0.35);
  opacity: 0;
  transform: translateY(8px);
  transition: opacity 0.18s ease, transform 0.18s ease, background-color 0.12s ease;
}

/* Optical centering for the play triangle only -- padding-right shrinks
   the centering space on the right side, nudging the icon left to
   compensate for the triangle's visual weight sitting right of center.
   The pause icon (two symmetric bars) doesn't need this and looked
   off-center with it applied unconditionally. */
.play-overlay.play-icon {
  padding-right: 2.5px;
}

.play-overlay:hover:not(:disabled) {
  background: var(--accent-hover);
}

.play-overlay:disabled {
  cursor: default;
  opacity: 0.6;
}

.playlist-tile:hover .play-overlay,
.playlist-tile:focus-visible .play-overlay,
.play-overlay.visible {
  opacity: 1;
  transform: translateY(0);
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

.tile-meta {
  font-size: 0.78rem;
  color: var(--muted);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>
