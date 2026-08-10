<script lang="ts">
  import type { Profile, Track } from "../types";
  import { formatDuration, isPlayable } from "../types";
  import { player } from "../stores/player.svelte";
  import { likes } from "../stores/likes.svelte";
  import { following } from "../stores/following.svelte";
  import { openUrl } from "@tauri-apps/plugin-opener";
  import ContextMenu from "./ContextMenu.svelte";
  import AddToPlaylistModal from "./AddToPlaylistModal.svelte";
  import Icon from "./Icon.svelte";
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";

  let {
    track,
    queue = [],
    index,
    onOpenProfile,
    onOpenTrack,
    me = null,
    onRemoveFromPlaylist,
    onOpenedOnSoundCloud,
  }: {
    track: Track;
    queue?: Track[];
    index?: number;
    onOpenProfile?: (id: number) => void;
    onOpenTrack?: (t: Track) => void;
    /** Logged-in user, needed only to gate the "Add to playlist" menu item -- omit to hide it. */
    me?: Profile | null;
    /** Only passed from PlaylistDetail, for a playlist the viewer owns. */
    onRemoveFromPlaylist?: (t: Track) => void;
    /** Fired when a like/follow decline or failure falls back to opening the track on soundcloud.com. */
    onOpenedOnSoundCloud?: () => void;
  } = $props();

  function openArtist(e: MouseEvent) {
    if (!track.user || !onOpenProfile) return;
    e.stopPropagation();
    onOpenProfile(track.user.id);
  }

  function openTrack(e: MouseEvent) {
    if (!onOpenTrack) return;
    e.stopPropagation();
    onOpenTrack(track);
  }

  let isCurrent = $derived(player.current?.id === track.id);
  let isPlaying = $derived(isCurrent && player.isPlaying);
  let playable = $derived(isPlayable(track));

  function handleRowClick() {
    if (playable) player.play(track, queue);
  }

  // Enter only, not Space, below: Space is the global play/pause shortcut
  // (PlayerBar's window-level handler). Handling it here too meant a
  // focused row would both re-trigger play() on itself and toggle pause
  // globally for the same keypress.

  let menuPos = $state<{ x: number; y: number } | null>(null);
  let showAddToPlaylist = $state(false);

  function openMenu(e: MouseEvent) {
    e.preventDefault();
    menuPos = { x: e.clientX, y: e.clientY };
  }

  // WebView2 doesn't reliably deliver the `contextmenu` DOM event once its
  // own native context menu is disabled at the settings level (confirmed
  // live: with AreDefaultContextMenusEnabled off, oncontextmenu simply
  // never fires here, so the app's own menu never opened either -- worse
  // than the native menu winning, since now nothing did). `mousedown` is a
  // plain pointer event with no such native-menu-pipeline entanglement, so
  // it's the reliable trigger; `oncontextmenu` stays wired too as a no-cost
  // fallback for platforms/runtimes where it does fire normally.
  function onRowMouseDown(e: MouseEvent) {
    if (e.button === 2) openMenu(e);
  }

  let isLiked = $derived(likes.has(track.id));
  let isFollowingArtist = $derived(track.user ? following.has(track.user.id) : false);

  // The connect -> write -> update-store flow lives on the shared
  // likes/following stores (see their doc comments); this just decides
  // what to do on decline/failure, same pattern as PlayerBar/TrackDetail.
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

<div
  class="row"
  class:active={isCurrent}
  class:unplayable={!playable}
  role="button"
  tabindex="0"
  onclick={handleRowClick}
  onkeydown={(e) => e.key === "Enter" && handleRowClick()}
  oncontextmenu={openMenu}
  onmousedown={onRowMouseDown}
>
  {#if index !== undefined}
    <span class="index">
      {#if !playable}
        <Icon name="lock" size={11} />
      {:else if isCurrent}
        <Icon name={isPlaying ? "pause" : "play"} size={12} />
      {:else}
        {index + 1}
      {/if}
    </span>
  {/if}
  <span class="artwork">
    {#if track.artwork_url}
      <img src={track.artwork_url} alt="" loading="lazy" />
    {:else}
      <span class="artwork-fallback"><Icon name="music" size={14} /></span>
    {/if}
    <span class="play-overlay"><Icon name={playable ? (isPlaying ? "pause" : "play") : "lock"} size={14} /></span>
  </span>
  <span class="info">
    {#if onOpenTrack}
      <button class="title title-btn" onclick={openTrack}>{track.title ?? `Untitled #${track.id}`}</button>
    {:else}
      <span class="title">{track.title ?? `Untitled #${track.id}`}</span>
    {/if}
    <button class="artist" onclick={openArtist}>{track.user?.username ?? "Unknown artist"}</button>
  </span>
  <span class="duration">{formatDuration(track.duration)}</span>
</div>

{#if menuPos}
  <ContextMenu x={menuPos.x} y={menuPos.y} items={menuItems} onClose={() => (menuPos = null)} />
{/if}

{#if showAddToPlaylist}
  <AddToPlaylistModal {track} {me} onClose={() => (showAddToPlaylist = false)} />
{/if}

<style>
.row {
  display: flex;
  align-items: center;
  gap: 0.85rem;
  width: 100%;
  padding: 0.45rem 0.6rem;
  background: none;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  text-align: left;
  color: inherit;
  font: inherit;
  transition: background-color 0.12s ease;
}

.row:hover {
  background: var(--row-hover);
}

.row.active .title {
  color: var(--accent);
}

.index {
  width: 1.5rem;
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--muted);
  font-size: 0.85rem;
}

.artwork {
  position: relative;
  width: 44px;
  height: 44px;
  flex-shrink: 0;
  border-radius: 5px;
  overflow: hidden;
  background: var(--artwork-bg);
  display: flex;
  align-items: center;
  justify-content: center;
}

.artwork img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.artwork-fallback {
  color: var(--muted);
  font-size: 1.1rem;
}

.play-overlay {
  position: absolute;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.45);
  color: white;
  opacity: 0;
  font-size: 0.9rem;
  transition: opacity 0.15s ease;
}

.row:hover .play-overlay,
.row.active .play-overlay,
.row.unplayable .play-overlay {
  opacity: 1;
}

.row.unplayable {
  cursor: default;
}

.row.unplayable .title,
.row.unplayable .index {
  color: var(--muted);
}

.info {
  display: flex;
  flex-direction: column;
  min-width: 0;
  flex: 1;
}

.title {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-weight: 600;
  font-size: 0.92rem;
}

.title-btn {
  background: none;
  border: none;
  padding: 0;
  color: inherit;
  font: inherit;
  cursor: pointer;
  text-align: left;
  width: fit-content;
  max-width: 100%;
}

.title-btn:hover {
  text-decoration: underline;
}

.artist {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: 0.8rem;
  color: var(--muted);
  width: fit-content;
  background: none;
  border: none;
  padding: 0;
  font: inherit;
  cursor: pointer;
  text-align: left;
}

.artist:hover {
  text-decoration: underline;
  color: var(--fg);
}

.duration {
  font-size: 0.8rem;
  color: var(--muted);
  flex-shrink: 0;
  font-variant-numeric: tabular-nums;
}
</style>
