<script lang="ts">
  import { player } from "../stores/player.svelte";
  import { api } from "../api";
  import { likes } from "../stores/likes.svelte";
  import { following } from "../stores/following.svelte";
  import { formatDuration } from "../types";
  import type { MiniPlayerCommand, MiniPlayerState, Track } from "../types";
  import Icon from "./Icon.svelte";
  import FollowButton from "./FollowButton.svelte";
  import { openUrl } from "@tauri-apps/plugin-opener";
  import { emit, listen } from "@tauri-apps/api/event";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { untrack } from "svelte";

  let {
    onOpenTrack,
    onOpenProfile,
    onOpenedOnSoundCloud,
  }: { onOpenTrack: (t: Track) => void; onOpenProfile: (id: number) => void; onOpenedOnSoundCloud?: () => void } = $props();

  let audioEl: HTMLAudioElement;
  // Position/duration live on the store now (player.currentTime/player.duration)
  // -- see player.svelte.ts's onTimeUpdate/onDurationChange/seek/watchdog --
  // so they're consistently correct for both this window's own scrubber and
  // whatever reads them for the mini player, and are unit-testable without
  // needing a real <audio> element or a mounted component.
  let isPlaying = $state(false);
  let showQueue = $state(false);
  let dragIndex = $state<number | null>(null);
  // player.currentTime only updates a few times a second (see
  // onTimeUpdate/the watchdog in player.svelte.ts), and the scrubber fill
  // had no CSS transition -- every tick just teleported the width straight
  // to the new value, which reads as choppy/"splotchy" next to anything
  // actually smooth. Same fix as the mini player's waveform reveal: snap
  // instantly to the true position (no transition) whenever it changes,
  // then hand a single transition all the way to the track's end off to
  // the compositor -- cheap (no per-frame JS at all) and re-syncs
  // automatically on every update, so drift never has room to accumulate.
  let scrubberFillPct = $state(0);
  let scrubberTransitionSec = $state(0);

  $effect(() => {
    const duration = player.duration;
    const currentTime = player.currentTime;
    if (duration <= 0) {
      scrubberTransitionSec = 0;
      scrubberFillPct = 0;
      return;
    }
    scrubberTransitionSec = 0;
    scrubberFillPct = Math.min(100, (currentTime / duration) * 100);
    if (!isPlaying) return;
    const remainingSeconds = Math.max(0, duration - currentTime);
    const raf = requestAnimationFrame(() => {
      scrubberTransitionSec = remainingSeconds;
      scrubberFillPct = 100;
    });
    return () => cancelAnimationFrame(raf);
  });
  let isLiked = $derived(player.current ? likes.has(player.current.id) : false);
  let isFollowing = $derived(player.current?.user ? following.has(player.current.user.id) : false);

  // Unofficial-API like/follow writes are DataDome-blocked (confirmed
  // live), but official OAuth works cleanly -- see docs/oauth-migration.md.
  // The connect -> write -> update-store flow itself lives on the
  // likes/following stores (shared with TrackDetail.svelte and TrackRow's
  // context menu); this just decides what to do on decline/failure.
  async function toggleLike() {
    const track = player.current;
    if (!track) return;
    try {
      if ((await likes.toggle(track)) === "declined") openOnSoundCloud();
    } catch (e) {
      player.error = `Failed to ${likes.has(track.id) ? "unlike" : "like"} track: ${e}`;
    }
  }

  function openOnSoundCloud() {
    if (!player.current?.permalink_url) return;
    openUrl(player.current.permalink_url);
    onOpenedOnSoundCloud?.();
  }

  /** Mirrors toggleLike() above, for the mini player's "toggleFollow" command -- see the emitter/listener effects below. */
  async function toggleFollowForCurrent() {
    const user = player.current?.user;
    if (!user) return;
    try {
      if ((await following.toggle(user)) === "declined") openOnSoundCloud();
    } catch (e) {
      player.error = `Failed to ${following.has(user.id) ? "unfollow" : "follow"}: ${e}`;
    }
  }

  function miniPlayerState(): MiniPlayerState {
    return {
      track: player.current,
      isPlaying,
      position: player.currentTime,
      duration: player.duration,
      shuffle: player.shuffle,
      loop: player.loop,
      isLiked,
      isFollowing,
      volume: player.volume,
      waveform: waveformSamples,
      upcoming: player.upcoming.slice(0, 8),
    };
  }

  function dispatchMiniCommand(command: MiniPlayerCommand) {
    switch (command.action) {
      case "toggle": player.toggle(); break;
      case "next": player.next(); break;
      case "previous": player.previous(); break;
      case "shuffle": player.toggleShuffle(); break;
      case "cycleLoop": player.cycleLoop(); break;
      case "toggleLike": toggleLike(); break;
      case "toggleFollow": toggleFollowForCurrent(); break;
      case "seek":
        player.seek(command.position);
        emit("player:state", miniPlayerState());
        break;
      case "toggleMute": player.toggleMute(); break;
      case "setVolume": player.setVolume(command.value); break;
    }
  }

  // The mini player's progress bar shows SoundCloud's own per-track
  // waveform (the same amplitude-envelope data their web player draws),
  // not a live spectrum analyser -- confirmed live that Track.waveform_url
  // (wave.sndcdn.com/{id}_m.json, {width, height, samples: number[]}) is
  // public and CORS-open (Access-Control-Allow-Origin: *), so this fetches
  // it directly rather than needing a Rust-side proxy. Static per track,
  // so it only needs to be (re)loaded on track change, not polled.
  const MINI_WAVE_BARS = 28;
  let waveformSamples = $state<number[]>([]);
  let waveformTrackId: number | null = null;

  async function loadWaveform(track: Track | null) {
    if (!track?.waveform_url) {
      waveformSamples = [];
      waveformTrackId = track?.id ?? null;
      return;
    }
    const trackId = track.id;
    waveformTrackId = trackId;
    try {
      const resp = await fetch(track.waveform_url);
      if (!resp.ok) throw new Error(`HTTP ${resp.status}`);
      const data: { height?: number; samples?: number[] } = await resp.json();
      const samples = data.samples ?? [];
      if (waveformTrackId !== trackId || samples.length === 0) return; // track changed again mid-fetch
      const peak = data.height ?? Math.max(1, ...samples);
      waveformSamples = resampleWaveform(samples, peak, MINI_WAVE_BARS);
      emit("player:state", miniPlayerState());
    } catch {
      if (waveformTrackId === trackId) waveformSamples = [];
    }
  }

  /** Averages `samples` down into `buckets` values normalized to 0..1. */
  function resampleWaveform(samples: number[], peak: number, buckets: number): number[] {
    const out: number[] = [];
    const bucketSize = samples.length / buckets;
    for (let i = 0; i < buckets; i++) {
      const start = Math.floor(i * bucketSize);
      const end = Math.max(start + 1, Math.floor((i + 1) * bucketSize));
      let sum = 0;
      let count = 0;
      for (let j = start; j < end && j < samples.length; j++) {
        sum += samples[j];
        count++;
      }
      out.push(count > 0 ? Math.min(1, sum / count / peak) : 0);
    }
    return out;
  }

  $effect(() => {
    loadWaveform(player.current);
  });

  // Mount-only: this must run exactly once, but attach() internally reads
  // player.volume (`el.volume = this.volume`) -- without `untrack`, Svelte
  // counts that as a dependency of THIS effect too (the same bug class as
  // the structural effect below), so it was silently re-running on every
  // single volume tick. That re-ran restore(), which re-imports whatever
  // position was last persisted to localStorage into pendingSeek -- and
  // since the currentTime getter always prefers pendingSeek when it's
  // non-null, that permanently pinned the displayed position at that stale
  // value even though the real audioEl.currentTime kept ticking along fine
  // underneath. Confirmed live (debug logging, since removed): this, not a
  // stalled timeupdate or an IPC flood, was the actual volume/progress-bar
  // freeze.
  $effect(() => {
    untrack(() => {
      player.attach(audioEl);
      player.restore();
    });
    return () => player.destroy();
  });
  $effect(() => {
    player.isPlaying = isPlaying;
  });

  // Browsers throttle background timers uniformly (ontimeupdate included),
  // so a backgrounded window's position can lag until the next throttled
  // tick lands (the watchdog in player.svelte.ts covers the steady-state
  // case, but that's a once-a-second poll -- no need to wait on it here).
  // Force an immediate resync the moment the window becomes visible again.
  $effect(() => {
    function resync() {
      if (document.visibilityState === "visible") player.onTimeUpdate();
    }
    document.addEventListener("visibilitychange", resync);
    return () => document.removeEventListener("visibilitychange", resync);
  });

  // Structural changes (track switch, play/pause, shuffle, loop, like,
  // follow, queue contents) push a fresh snapshot to the mini player right
  // away. Position/duration/volume piggyback on whatever the most recent
  // snapshot was rather than triggering their own emit here -- the interval
  // effect below covers live scrubber movement instead, so the mini player
  // isn't getting a full cross-window IPC event on every position tick (or,
  // confirmed live, dozens of times a second while the volume slider is
  // being dragged: miniPlayerState() reads player.volume/currentTime/
  // duration internally, and without `untrack`, Svelte counts those as
  // dependencies of THIS effect too since it reads them synchronously --
  // silently turning "structural changes only" into "also volume changes,
  // also every position tick", flooding the IPC channel and starving
  // everything else on the event loop long enough to look like a freeze).
  $effect(() => {
    player.current;
    isPlaying;
    player.shuffle;
    player.loop;
    isLiked;
    isFollowing;
    player.upcoming.length;
    untrack(() => emit("player:state", miniPlayerState()));
  });

  $effect(() => {
    const id = setInterval(() => {
      if (isPlaying) emit("player:state", miniPlayerState());
    }, 1000);
    return () => clearInterval(id);
  });

  $effect(() => {
    let unlistenRequest: (() => void) | undefined;
    let unlistenCommand: (() => void) | undefined;
    let unlistenShowMain: (() => void) | undefined;
    listen("miniplayer:request-state", () => emit("player:state", miniPlayerState())).then((f) => (unlistenRequest = f));
    listen<MiniPlayerCommand>("player:command", (e) => dispatchMiniCommand(e.payload)).then((f) => (unlistenCommand = f));
    listen<{ trackId: number | null }>("miniplayer:show-main", (e) => {
      // The mini player only hides itself (see MiniPlayer.svelte's
      // backToApp) -- it has no direct way to show/focus a DIFFERENT
      // window, so the main window brings itself forward in response to
      // this instead. This used to piggyback on the mini player's
      // CloseRequested handler in src-tauri/src/lib.rs, but that stopped
      // firing once the mini player switched from close() to hide().
      const win = getCurrentWindow();
      win.show();
      win.setFocus();
      const track = e.payload.trackId !== null ? player.queue.find((t) => t.id === e.payload.trackId) : null;
      if (track) onOpenTrack(track);
    }).then((f) => (unlistenShowMain = f));
    return () => {
      unlistenRequest?.();
      unlistenCommand?.();
      unlistenShowMain?.();
    };
  });

  function seek(e: MouseEvent) {
    if (!player.duration) return;
    const bar = e.currentTarget as HTMLElement;
    const rect = bar.getBoundingClientRect();
    const pct = (e.clientX - rect.left) / rect.width;
    player.seek(pct * player.duration);
    emit("player:state", miniPlayerState());
  }

  function onVolumeInput(e: Event) {
    player.setVolume(Number((e.target as HTMLInputElement).value));
  }

  // Range inputs (the volume slider) are deliberately NOT treated as a
  // typing target -- dragging it with the mouse leaves it focused, and
  // without this exception space would silently stop toggling playback
  // right after you touch the volume, with no visible cause. Real text
  // entry (search boxes, comments, playlist names) still blocks it.
  function isTypingTarget(el: EventTarget | null): boolean {
    if (!(el instanceof HTMLElement)) return false;
    if (el.isContentEditable || el.tagName === "TEXTAREA") return true;
    if (el.tagName === "INPUT") return (el as HTMLInputElement).type !== "range";
    return false;
  }

  function onGlobalKeydown(e: KeyboardEvent) {
    if (e.code !== "Space" || isTypingTarget(e.target)) return;
    e.preventDefault();
    player.toggle();
  }

  function dragStart(e: DragEvent, absIndex: number) {
    dragIndex = absIndex;
    // Required by the HTML5 DnD spec for the drop to reliably fire, even
    // though we only use our own dragIndex state to actually reorder.
    e.dataTransfer?.setData("text/plain", String(absIndex));
    if (e.dataTransfer) e.dataTransfer.effectAllowed = "move";
  }

  function dragOver(e: DragEvent) {
    e.preventDefault();
  }

  function drop(absIndex: number) {
    if (dragIndex !== null && dragIndex !== absIndex) {
      player.reorderQueue(dragIndex, absIndex);
    }
    dragIndex = null;
  }
</script>

<svelte:window onkeydown={onGlobalKeydown} />

<div class="player-bar">
  <audio
    bind:this={audioEl}
    ontimeupdate={() => player.onTimeUpdate()}
    ondurationchange={() => player.onDurationChange()}
    onplay={() => (isPlaying = true)}
    onpause={() => { isPlaying = false; player.savePositionTick(true); }}
    onended={() => player.onTrackEnded()}
    onerror={() => (player.error = audioEl.error?.message ?? "playback error")}
  ></audio>

  <button class="scrubber" onclick={seek} aria-label="Seek" disabled={!player.current}>
    <span class="scrubber-fill" style="width: {scrubberFillPct}%; transition: width {scrubberTransitionSec}s linear;"></span>
  </button>

  {#if showQueue}
    <div class="queue-panel">
      <div class="queue-header">
        <span>Up next</span>
        <button class="close" onclick={() => (showQueue = false)} aria-label="Close queue"><Icon name="close" size={12} /></button>
      </div>
      {#if player.upcoming.length === 0}
        <p class="queue-empty">Nothing queued.</p>
      {:else}
        <ul>
          {#each player.upcoming as t, i}
            {@const absIndex = player.queueIndex + 1 + i}
            <li
              draggable="true"
              class:dragging={dragIndex === absIndex}
              ondragstart={(e) => dragStart(e, absIndex)}
              ondragover={dragOver}
              ondrop={() => drop(absIndex)}
              ondragend={() => (dragIndex = null)}
            >
              <span class="drag-handle"><Icon name="grip" size={12} /></span>
              <span class="qtitle">{t.title ?? `Track #${t.id}`}</span>
              <span class="qartist">{t.user?.username ?? ""}</span>
              <button class="qremove" onclick={() => player.removeFromQueue(absIndex)} aria-label="Remove from queue"><Icon name="close" size={11} /></button>
            </li>
          {/each}
        </ul>
      {/if}
    </div>
  {/if}

  <div class="content">
    <div class="track-info">
      {#if player.current}
        <button class="artwork-btn" onclick={() => onOpenTrack(player.current!)} aria-label="View track">
          {#if player.current.artwork_url}
            <img src={player.current.artwork_url} alt="" class="artwork" />
          {:else}
            <div class="artwork artwork-fallback"><Icon name="music" size={14} /></div>
          {/if}
        </button>
        <div class="text">
          <button class="title" onclick={() => onOpenTrack(player.current!)}>{player.current.title ?? `Track #${player.current.id}`}</button>
          {#if player.current.user}
            <button class="artist" onclick={() => onOpenProfile(player.current!.user!.id)}>{player.current.user.username ?? ""}</button>
          {/if}
        </div>
      {:else}
        <span class="muted">Nothing playing</span>
      {/if}
    </div>

    <div class="controls">
      <button
        class="toggle-btn"
        class:active={player.shuffle}
        onclick={() => player.toggleShuffle()}
        aria-label="Shuffle"
        title="Shuffle"
      >
        <Icon name="shuffle" size={14} />
      </button>
      <button onclick={() => player.previous()} disabled={!player.current} aria-label="Previous"><Icon name="skip-back" /></button>
      <button class="play" onclick={() => player.toggle()} disabled={!player.current} aria-label="Play/Pause">
        <Icon name={isPlaying ? "pause" : "play"} size={18} />
      </button>
      <button onclick={() => player.next()} disabled={!player.current} aria-label="Next"><Icon name="skip-forward" /></button>
      <button
        class="toggle-btn"
        class:active={player.loop !== "off"}
        onclick={() => player.cycleLoop()}
        aria-label="Loop"
        title={player.loop === "one" ? "Repeat one" : player.loop === "all" ? "Repeat all" : "Repeat off"}
      >
        <Icon name="repeat" size={14} />
        {#if player.loop === "one"}<span class="loop-one-badge">1</span>{/if}
      </button>
    </div>

    <div class="right-controls">
      <button
        class="like-btn"
        class:active={isLiked}
        onclick={toggleLike}
        disabled={!player.current || likes.isBusy(player.current.id)}
        aria-label={isLiked ? "Unlike" : "Like"}
        title={isLiked ? "Unlike" : "Like"}
      >
        <Icon name={isLiked ? "heart-filled" : "heart"} size={15} />
      </button>
      {#if player.current?.user}
        <FollowButton userId={player.current.user.id} permalinkUrl={player.current.user.permalink_url} compact />
      {/if}
      <button
        class="cloud-btn"
        onclick={openOnSoundCloud}
        disabled={!player.current}
        aria-label="Open on SoundCloud"
        title="Open on SoundCloud"
      >
        <Icon name="cloud" size={15} />
      </button>
      <div class="time-display">
        <span>{formatDuration(player.currentTime * 1000)}</span>
        <span class="sep">/</span>
        <span>{formatDuration(player.duration * 1000)}</span>
      </div>
      <button class="queue-toggle" class:active={showQueue} onclick={() => (showQueue = !showQueue)} aria-label="Toggle queue"><Icon name="queue" size={15} /></button>
      <button class="queue-toggle" onclick={() => api.openMiniPlayer()} aria-label="Mini player" title="Mini player"><Icon name="pip" size={15} /></button>
      <div class="volume">
        <button class="vol-icon" onclick={() => player.toggleMute()} aria-label={player.volume === 0 ? "Unmute" : "Mute"} title={player.volume === 0 ? "Unmute" : "Mute"}>
          <Icon name={player.volume === 0 ? "volume-mute" : "volume"} size={15} />
        </button>
        <input type="range" min="0" max="1" step="0.01" value={player.volume} oninput={onVolumeInput} aria-label="Volume" />
      </div>
    </div>
  </div>

  {#if player.error}
    <div class="error">{player.error}</div>
  {:else if player.notice}
    <div class="notice">{player.notice}</div>
  {/if}
</div>

<style>
.player-bar {
  position: relative;
  background: var(--player-bg);
  color: var(--player-fg);
  flex-shrink: 0;
}

.scrubber {
  width: 100%;
  height: 4px;
  padding: 0;
  border: none;
  background: rgba(255, 255, 255, 0.15);
  cursor: pointer;
  display: block;
}

.scrubber:disabled {
  cursor: default;
}

.scrubber-fill {
  display: block;
  height: 100%;
  background: var(--accent);
}

.content {
  display: grid;
  grid-template-columns: 1fr auto 1fr;
  align-items: center;
  gap: 1rem;
  padding: 0.65rem 1.25rem;
}

.track-info {
  display: flex;
  align-items: center;
  gap: 0.6rem;
  min-width: 0;
}

.artwork-btn {
  background: none;
  border: none;
  padding: 0;
  cursor: pointer;
  flex-shrink: 0;
}

.artwork {
  width: 40px;
  height: 40px;
  border-radius: 3px;
  object-fit: cover;
  display: block;
}

.artwork-fallback {
  display: flex;
  align-items: center;
  justify-content: center;
  background: #333;
  color: #888;
}

.text {
  display: flex;
  flex-direction: column;
  min-width: 0;
}

.title,
.artist {
  background: none;
  border: none;
  padding: 0;
  cursor: pointer;
  text-align: left;
  font: inherit;
  color: #e8e8e8;
}

.title {
  font-weight: 600;
  font-size: 0.9rem;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.title:hover,
.artist:hover {
  text-decoration: underline;
}

.artist {
  font-size: 0.78rem;
  color: #a0a0a0;
}

.muted {
  color: #a0a0a0;
}

.controls {
  display: flex;
  align-items: center;
  gap: 1rem;
  justify-self: center;
}

.controls button {
  display: flex;
  align-items: center;
  justify-content: center;
  background: none;
  border: none;
  cursor: pointer;
  color: #e8e8e8;
  padding: 0.25rem;
}

.toggle-btn {
  position: relative;
  color: #999 !important;
}

.toggle-btn:hover {
  color: #ccc !important;
}

.toggle-btn.active {
  color: var(--accent) !important;
}

.loop-one-badge {
  position: absolute;
  top: -2px;
  right: -4px;
  font-size: 0.55rem;
  font-weight: 700;
  line-height: 1;
  background: var(--accent);
  color: white;
  border-radius: 50%;
  width: 11px;
  height: 11px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.controls button:disabled {
  opacity: 0.35;
  cursor: default;
}

.controls .play {
  width: 2.2rem;
  height: 2.2rem;
  border-radius: 50%;
  background: white;
  color: #111;
  display: flex;
  align-items: center;
  justify-content: center;
}

.controls .play:disabled {
  background: #555;
  color: #999;
}

.right-controls {
  display: flex;
  align-items: center;
  gap: 0.9rem;
  justify-self: end;
}

.time-display {
  font-size: 0.75rem;
  color: #a0a0a0;
  font-variant-numeric: tabular-nums;
  white-space: nowrap;
}

.sep {
  margin: 0 0.2em;
}

.queue-toggle {
  display: flex;
  align-items: center;
  background: none;
  border: none;
  cursor: pointer;
  color: #a0a0a0;
  padding: 0.3rem;
  border-radius: 4px;
}

.queue-toggle:hover,
.queue-toggle.active {
  color: white;
  background: rgba(255, 255, 255, 0.1);
}

.like-btn {
  display: flex;
  align-items: center;
  background: none;
  border: none;
  cursor: pointer;
  color: #a0a0a0;
  padding: 0.3rem;
  border-radius: 4px;
}

.like-btn:hover {
  color: white;
  background: rgba(255, 255, 255, 0.1);
}

.like-btn.active {
  color: var(--accent);
}

.like-btn:disabled {
  opacity: 0.35;
  cursor: default;
}

.cloud-btn {
  display: flex;
  align-items: center;
  background: none;
  border: none;
  cursor: pointer;
  color: var(--accent);
  padding: 0.3rem;
  border-radius: 4px;
}

.cloud-btn:hover {
  background: rgba(255, 85, 0, 0.15);
}

.cloud-btn:disabled {
  opacity: 0.35;
  cursor: default;
}

.volume {
  display: flex;
  align-items: center;
  gap: 0.4rem;
}

.vol-icon {
  display: flex;
  align-items: center;
  background: none;
  border: none;
  padding: 0.2rem;
  border-radius: 4px;
  color: #a0a0a0;
  cursor: pointer;
}

.vol-icon:hover {
  color: white;
  background: rgba(255, 255, 255, 0.1);
}

.volume input[type="range"] {
  width: 80px;
  accent-color: var(--accent);
}

.error,
.notice {
  position: absolute;
  bottom: 100%;
  left: 1.25rem;
  right: 1.25rem;
  padding: 0.4rem 0.75rem;
  border-radius: 6px;
  font-size: 0.85rem;
  margin-bottom: 0.5rem;
}

.error {
  background: var(--error-bg);
  color: var(--error-text);
}

.notice {
  background: var(--titlebar-bg);
  color: var(--titlebar-fg-muted);
}

.queue-panel {
  position: absolute;
  bottom: 100%;
  right: 1.25rem;
  width: 20rem;
  max-height: 22rem;
  overflow-y: auto;
  background: #161616;
  border: 1px solid #2a2a2a;
  border-radius: 8px 8px 0 0;
  box-shadow: 0 -8px 24px rgba(0, 0, 0, 0.4);
}

.queue-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.6rem 0.8rem;
  border-bottom: 1px solid #2a2a2a;
  font-size: 0.85rem;
  font-weight: 600;
  position: sticky;
  top: 0;
  background: #161616;
}

.queue-header .close {
  display: flex;
  align-items: center;
  background: none;
  border: none;
  color: #a0a0a0;
  cursor: pointer;
}

.queue-empty {
  padding: 1rem 0.8rem;
  color: #888;
  font-size: 0.85rem;
  margin: 0;
}

.queue-panel ul {
  list-style: none;
  margin: 0;
  padding: 0.3rem;
}

.queue-panel li {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.4rem 0.5rem;
  border-radius: 5px;
  font-size: 0.82rem;
  cursor: grab;
}

.queue-panel li:hover {
  background: rgba(255, 255, 255, 0.06);
}

.queue-panel li.dragging {
  opacity: 0.4;
}

.drag-handle {
  display: flex;
  align-items: center;
  color: #666;
  flex-shrink: 0;
}

.qtitle {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  max-width: 10rem;
}

.qartist {
  color: #888;
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.qremove {
  display: flex;
  align-items: center;
  background: none;
  border: none;
  color: #888;
  cursor: pointer;
  flex-shrink: 0;
}

.qremove:hover {
  color: white;
}
</style>
