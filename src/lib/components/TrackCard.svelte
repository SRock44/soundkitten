<script lang="ts">
  import { player } from "../stores/player.svelte";
  import { likes } from "../stores/likes.svelte";
  import { following } from "../stores/following.svelte";
  import { activeContextMenu } from "../stores/activeContextMenu.svelte";
  import { formatDuration, isPlayable } from "../types";
  import type { Profile, Track } from "../types";
  import { openUrl } from "@tauri-apps/plugin-opener";
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import ContextMenu from "./ContextMenu.svelte";
  import AddToPlaylistModal from "./AddToPlaylistModal.svelte";
  import Icon from "./Icon.svelte";

  let {
    track,
    queue = [],
    onOpenProfile,
    onOpenTrack,
    me = null,
    onRemoveFromPlaylist,
    onOpenedOnSoundCloud,
  }: {
    track: Track;
    queue?: Track[];
    onOpenProfile?: (id: number) => void;
    onOpenTrack?: (t: Track) => void;
    /** Logged-in user, needed only to gate the "Add to playlist" menu item -- omit to hide it. */
    me?: Profile | null;
    /** Only passed where a track can be removed from the playlist it's shown in. */
    onRemoveFromPlaylist?: (t: Track) => void;
    /** Fired when a like/follow decline or failure falls back to opening the track on soundcloud.com. */
    onOpenedOnSoundCloud?: () => void;
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

  // Same context-menu machinery as TrackRow.svelte -- see its comments for
  // the reasoning (shared activeContextMenu token so only one menu is ever
  // open at once, a click-triggered fallback since right-click/contextmenu
  // doesn't always cooperate with WebView2).
  let menuPos = $state<{ x: number; y: number } | null>(null);
  let showAddToPlaylist = $state(false);
  let myMenuToken = $state(0);
  let menuOpen = $derived(menuPos !== null && myMenuToken === activeContextMenu.token);

  function openMenu(e: MouseEvent) {
    e.preventDefault();
    myMenuToken = activeContextMenu.open();
    menuPos = { x: e.clientX, y: e.clientY };
  }

  function openMenuAtButton(e: MouseEvent) {
    e.stopPropagation();
    myMenuToken = activeContextMenu.open();
    const rect = (e.currentTarget as HTMLElement).getBoundingClientRect();
    menuPos = { x: rect.right, y: rect.bottom + 4 };
  }

  let isLiked = $derived(likes.has(track.id));
  let isFollowingArtist = $derived(track.user ? following.has(track.user.id) : false);

  async function toggleLike() {
    try {
      if ((await likes.toggle(track)) === "declined" && track.permalink_url) {
        openUrl(track.permalink_url);
        onOpenedOnSoundCloud?.();
      }
    } catch (e) {
      player.error = `Failed to ${isLiked ? "unlike" : "like"} track: ${e}`;
    }
  }

  async function toggleFollowArtist() {
    const user = track.user;
    if (!user) return;
    try {
      if ((await following.toggle(user)) === "declined" && user.permalink_url) {
        openUrl(user.permalink_url);
        onOpenedOnSoundCloud?.();
      }
    } catch (e) {
      player.error = `Failed to ${isFollowingArtist ? "unfollow" : "follow"}: ${e}`;
    }
  }

  const menuItems = $derived([
    ...(playable
      ? [
          { label: "Play now", onSelect: () => player.play(track, queue) },
          { label: "Play next", onSelect: () => player.playNext(track) },
          { label: "Add to queue", onSelect: () => player.addToQueue(track) },
        ]
      : []),
    ...(onOpenTrack ? [{ label: "View track", onSelect: () => onOpenTrack!(track) }] : []),
    { label: isLiked ? "Unlike" : "Like", onSelect: toggleLike },
    ...(track.user && onOpenProfile
      ? [{ label: "Go to artist", onSelect: () => onOpenProfile!(track.user!.id) }]
      : []),
    ...(track.user ? [{ label: isFollowingArtist ? "Unfollow artist" : "Follow artist", onSelect: toggleFollowArtist }] : []),
    ...(me ? [{ label: "Add to playlist...", onSelect: () => (showAddToPlaylist = true) }] : []),
    ...(onRemoveFromPlaylist ? [{ label: "Remove from playlist", danger: true, onSelect: () => onRemoveFromPlaylist!(track) }] : []),
    ...(track.permalink_url
      ? [{ label: "Copy link", onSelect: () => writeText(track.permalink_url!) }]
      : []),
  ]);
</script>

<div class="track-tile" role="button" tabindex="0" oncontextmenu={openMenu}>
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
    <button class="more-btn" onclick={openMenuAtButton} aria-label="More options" title="More options">
      <Icon name="more" size={14} />
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

{#if menuOpen && menuPos}
  <ContextMenu x={menuPos.x} y={menuPos.y} items={menuItems} onClose={() => (menuPos = null)} />
{/if}

{#if showAddToPlaylist}
  <AddToPlaylistModal {track} {me} onClose={() => (showAddToPlaylist = false)} />
{/if}

<style>
.track-tile {
  position: relative;
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

.more-btn {
  position: absolute;
  top: 1.35rem;
  right: 1.35rem;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 26px;
  height: 26px;
  background: rgba(0, 0, 0, 0.55);
  border: none;
  border-radius: 50%;
  color: white;
  cursor: pointer;
  opacity: 0;
  transition: opacity 0.15s ease, background-color 0.12s ease;
}

.more-btn:hover {
  background: rgba(0, 0, 0, 0.75);
}

.track-tile:hover .more-btn,
.track-tile:focus-within .more-btn {
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
