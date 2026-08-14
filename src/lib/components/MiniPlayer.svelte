<script lang="ts">
  import { listen, emit } from "@tauri-apps/api/event";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { settings } from "../stores/settings.svelte";
  import type { MiniPlayerCommand, MiniPlayerState } from "../types";
  import Icon from "./Icon.svelte";

  const WAVE_BARS = 28;

  let miniState = $state<MiniPlayerState>({
    track: null,
    isPlaying: false,
    position: 0,
    duration: 0,
    shuffle: false,
    loop: "off",
    isLiked: false,
    isFollowing: false,
    volume: 1,
    waveform: [],
    upcoming: [],
  });

  let showQueue = $state(false);
  let videoEl: HTMLVideoElement | undefined = $state();
  let waveEl: HTMLDivElement | undefined = $state();
  let dragging = $state(false);

  // `miniState.position` only updates once per "player:state" snapshot
  // (roughly once a second while playing, or immediately on pause/seek --
  // see PlayerBar.svelte). Rather than a separate JS-scheduled "snap then
  // animate to the end" dance (which had real bugs: rapid updates during a
  // drag-seek could cancel the scheduled animation before it ever started,
  // and any branch that got the snap/no-transition logic wrong -- e.g. on
  // pause -- would visibly jump to a stale value), revealPct is just a
  // plain derived value and the CSS transition (see .wave.overlay below)
  // is a fixed, always-on duration. The browser handles interruption on
  // its own: a new target value while a transition is in flight smoothly
  // redirects from wherever it currently is, rather than jumping -- so
  // this is simultaneously simpler AND self-correcting for every case
  // (routine ticks, pause, seek) with no special-casing needed.
  let revealPct = $derived(miniState.duration > 0 ? Math.min(100, (miniState.position / miniState.duration) * 100) : 0);

  // The one case that DOES need an instant, non-animated snap: switching
  // tracks (or the very first snapshot after opening) shouldn't visibly
  // slide over from the previous track's fill position.
  let lastTrackId: number | null | undefined = undefined;
  let instantReveal = $state(true);

  $effect(() => {
    const trackId = miniState.track?.id ?? null;
    if (trackId === lastTrackId) return;
    lastTrackId = trackId;
    instantReveal = true;
    const raf = requestAnimationFrame(() => (instantReveal = false));
    return () => cancelAnimationFrame(raf);
  });

  // This window has no <audio> element of its own (that stays in the main
  // window, see PlayerBar.svelte) and no seeded likes/following stores --
  // it's a thin remote control. "player:state" is the main window's
  // authoritative snapshot; "player:command" is how button presses here get
  // relayed back to the real stores over there. Both directions require
  // "mini-player" to be listed alongside "main" in
  // src-tauri/capabilities/default.json -- without that, Tauri's
  // permission system silently drops every listen()/emit() from this
  // window (no error, they just never arrive).
  $effect(() => {
    let unlisten: (() => void) | undefined;
    let cancelled = false;
    // Only send the request AFTER the listener is confirmed registered --
    // both listen() and emit() are async IPC round-trips with no ordering
    // guarantee between them, so emitting eagerly risks the main window's
    // reply arriving before anyone here is listening for it. That reply
    // never comes again on its own (the periodic re-sync in PlayerBar.svelte
    // only fires while a track is actively playing), so a lost first reply
    // meant this window could get stuck showing "Nothing playing" /
    // paused indefinitely even while the real track kept playing.
    listen<MiniPlayerState>("player:state", (e) => (miniState = e.payload)).then((f) => {
      if (cancelled) {
        f();
        return;
      }
      unlisten = f;
      emit("miniplayer:request-state");
    });
    return () => {
      cancelled = true;
      unlisten?.();
    };
  });

  // This window is only ever hidden, never destroyed (backToApp() above and
  // the Rust CloseRequested handler in src-tauri/src/lib.rs both just call
  // hide()), so closing it used to leave this <video> decoding and
  // compositing an invisible loop for as long as a track kept playing --
  // easily hours, since nothing ever told it the window went away. Track
  // page visibility here and fold it into the same play/pause effect below.
  let pageHidden = $state(document.visibilityState === "hidden");
  $effect(() => {
    function onVisibility() {
      pageHidden = document.visibilityState === "hidden";
    }
    document.addEventListener("visibilitychange", onVisibility);
    return () => document.removeEventListener("visibilitychange", onVisibility);
  });

  $effect(() => {
    if (!videoEl || settings.performanceMode) return;
    if (miniState.isPlaying && !pageHidden) {
      videoEl.play().catch(() => {});
    } else {
      videoEl.pause();
      // A paused mid-loop video freezes on whatever motion frame it happened
      // to be on (mid-jump, arms up...), which reads as "still dancing" even
      // though nothing's playing -- reset to the first frame so idle always
      // looks calm/at-rest instead. Only when actually paused, not merely
      // hidden -- resuming a still-playing track after reopening the window
      // should pick the loop back up, not restart it from frame 0.
      if (!miniState.isPlaying) videoEl.currentTime = 0;
    }
  });

  function send(command: MiniPlayerCommand) {
    emit("player:command", command);
  }

  async function backToApp(trackId: number | null = null) {
    // finally, not sequential awaits -- if the show-main emit ever rejects
    // (event permission hiccup, main window not listening yet, whatever),
    // the window must still hide instead of leaving the user stuck with
    // no visible way back into the app. Hide, not close: closing destroyed
    // the webview, so every reopen was a full fresh load that visibly
    // "restarted" (blank until the first state sync round-trip landed)
    // instead of instantly showing already-current state. The main
    // window's own listener for "miniplayer:show-main" (PlayerBar.svelte)
    // shows and focuses itself in response -- this window has no direct
    // way to do that for a window that isn't itself.
    try {
      await emit("miniplayer:show-main", { trackId });
    } finally {
      await getCurrentWindow().hide();
    }
  }

  function seekFromEvent(e: MouseEvent) {
    if (!waveEl || !miniState.duration) return;
    const rect = waveEl.getBoundingClientRect();
    const pct = Math.min(1, Math.max(0, (e.clientX - rect.left) / rect.width));
    send({ action: "seek", position: pct * miniState.duration });
  }

  function onWaveDown(e: MouseEvent) {
    if (!miniState.duration) return;
    dragging = true;
    seekFromEvent(e);
  }

  $effect(() => {
    if (!dragging) return;
    function onMove(e: MouseEvent) {
      seekFromEvent(e);
    }
    function onUp() {
      dragging = false;
    }
    window.addEventListener("mousemove", onMove);
    window.addEventListener("mouseup", onUp);
    return () => {
      window.removeEventListener("mousemove", onMove);
      window.removeEventListener("mouseup", onUp);
    };
  });

  function onVolumeInput(e: Event) {
    send({ action: "setVolume", value: Number((e.target as HTMLInputElement).value) });
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

  // This is a separate window/webview with its own JS process and its own
  // `window` object -- the main app's space-bar play/pause handler
  // (PlayerBar.svelte's onGlobalKeydown) only ever sees keydowns in ITS
  // window, so space did nothing at all here until this existed.
  function onGlobalKeydown(e: KeyboardEvent) {
    if (e.code !== "Space" || isTypingTarget(e.target)) return;
    e.preventDefault();
    send({ action: "toggle" });
  }

  // SoundCloud's real per-track waveform (see PlayerBar.svelte's
  // loadWaveform) -- a static amplitude-over-time shape, not a live
  // spectrum analyser, same as SoundCloud's own player draws. Flat
  // baseline when there's no data yet (still loading, or the track has no
  // waveform_url) rather than faking a shape.
  function barHeightPct(i: number): number {
    if (miniState.waveform.length === 0) return 12;
    return 12 + miniState.waveform[i % miniState.waveform.length] * 78;
  }
</script>

<svelte:window onkeydown={onGlobalKeydown} />

<div class="mini-player" data-tauri-drag-region>
  <div class="top-row" data-tauri-drag-region>
    <div class="visual" data-tauri-drag-region>
      {#if settings.performanceMode}
        <div class="visual-static" data-tauri-drag-region><Icon name="music" size={18} /></div>
      {:else}
        <video bind:this={videoEl} src="/cat-dance.mp4" muted loop playsinline data-tauri-drag-region></video>
      {/if}
    </div>

    <div class="info" data-tauri-drag-region>
      {#if miniState.track}
        <button class="title" onclick={() => backToApp(miniState.track!.id)}>{miniState.track.title ?? `Track #${miniState.track.id}`}</button>
        <button class="artist" onclick={() => backToApp(miniState.track!.id)}>{miniState.track.user?.username ?? ""}</button>
      {:else}
        <span class="muted" data-tauri-drag-region>Nothing playing</span>
      {/if}
    </div>

    <button class="icon-btn expand-btn" onclick={() => backToApp()} aria-label="Back to full app" title="Back to full app">
      <Icon name="pip" size={13} />
    </button>
  </div>

  {#if showQueue}
    <div class="queue-view">
      <div class="queue-header" data-tauri-drag-region>
        <span>Up next</span>
        <button class="icon-btn" onclick={() => (showQueue = false)} aria-label="Close queue"><Icon name="close" size={11} /></button>
      </div>
      {#if miniState.upcoming.length === 0}
        <p class="queue-empty">Nothing queued.</p>
      {:else}
        <ul>
          {#each miniState.upcoming as t, i}
            <!-- svelte-ignore a11y_no_noninteractive_element_to_interactive_role -->
            <li
              onclick={() => send({ action: "playFromQueue", index: i })}
              onkeydown={(e) => { if (e.key === "Enter" || e.key === " ") { e.preventDefault(); send({ action: "playFromQueue", index: i }); } }}
              role="button"
              tabindex="0"
              aria-label={`Play "${t.title ?? `Track #${t.id}`}" now`}
            >
              <span class="qtitle">{t.title ?? `Track #${t.id}`}</span>
              <span class="qartist">{t.user?.username ?? ""}</span>
            </li>
          {/each}
        </ul>
      {/if}
    </div>
  {:else}
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="progress-row">
      <div class="wave-stack" class:seekable={!!miniState.duration} bind:this={waveEl} onmousedown={onWaveDown}>
        <div class="wave" class:animated={miniState.isPlaying && !settings.performanceMode}>
          {#each { length: WAVE_BARS } as _, i}
            <span style="height: {barHeightPct(i)}%"></span>
          {/each}
        </div>
        <div
          class="wave overlay"
          class:animated={miniState.isPlaying && !settings.performanceMode}
          style="clip-path: inset(0 calc(100% - {revealPct}%) 0 0); transition: clip-path {instantReveal ? 0 : 1}s linear;"
        >
          {#each { length: WAVE_BARS } as _, i}
            <span style="height: {barHeightPct(i)}%"></span>
          {/each}
        </div>
      </div>
    </div>

    <div class="transport-row" data-tauri-drag-region>
      <button class="toggle-btn" class:active={miniState.shuffle} onclick={() => send({ action: "shuffle" })} aria-label="Shuffle" title="Shuffle">
        <Icon name="shuffle" size={13} />
      </button>
      <button class="transport-btn" onclick={() => send({ action: "previous" })} disabled={!miniState.track} aria-label="Previous">
        <Icon name="skip-back" size={16} />
      </button>
      <button class="play-btn" onclick={() => send({ action: "toggle" })} disabled={!miniState.track} aria-label="Play/Pause">
        <Icon name={miniState.isPlaying ? "pause" : "play"} size={18} />
      </button>
      <button class="transport-btn" onclick={() => send({ action: "next" })} disabled={!miniState.track} aria-label="Next">
        <Icon name="skip-forward" size={16} />
      </button>
      <button class="toggle-btn" class:active={miniState.loop !== "off"} onclick={() => send({ action: "cycleLoop" })} aria-label="Loop" title="Loop">
        <Icon name="repeat" size={13} />
        {#if miniState.loop === "one"}<span class="loop-one-badge">1</span>{/if}
      </button>
    </div>

    <div class="secondary-row" data-tauri-drag-region>
      <button class="icon-btn" class:active={miniState.isLiked} onclick={() => send({ action: "toggleLike" })} disabled={!miniState.track} aria-label="Like" title="Like">
        <Icon name={miniState.isLiked ? "heart-filled" : "heart"} size={13} />
      </button>
      <button class="icon-btn" class:active={miniState.isFollowing} onclick={() => send({ action: "toggleFollow" })} disabled={!miniState.track?.user} aria-label="Follow" title="Follow">
        <Icon name={miniState.isFollowing ? "user-filled" : "user"} size={13} />
      </button>
      <button class="icon-btn" class:active={showQueue} onclick={() => (showQueue = true)} aria-label="Queue" title="Queue">
        <Icon name="queue" size={13} />
      </button>
      <span class="row-spacer" data-tauri-drag-region></span>
      <button class="icon-btn" onclick={() => send({ action: "toggleMute" })} aria-label={miniState.volume === 0 ? "Unmute" : "Mute"} title={miniState.volume === 0 ? "Unmute" : "Mute"}>
        <Icon name={miniState.volume === 0 ? "volume-mute" : "volume"} size={13} />
      </button>
      <input class="mini-volume" type="range" min="0" max="1" step="0.01" value={miniState.volume} oninput={onVolumeInput} aria-label="Volume" />
    </div>
  {/if}
</div>

<style>
.mini-player {
  height: 100vh;
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
  gap: 0.2rem;
  padding: 0.4rem 0.5rem;
  background: var(--player-bg);
  color: var(--player-fg);
  overflow: hidden;
  user-select: none;
}

.top-row {
  display: flex;
  align-items: center;
  gap: 0.4rem;
}

.visual {
  position: relative;
  width: 36px;
  height: 36px;
  border-radius: 6px;
  overflow: hidden;
  flex-shrink: 0;
  background: var(--player-bg);
}

.visual video {
  width: 100%;
  height: 100%;
  object-fit: cover;
  display: block;
  /* The source video has a solid black backdrop baked into it (no real
     alpha channel to work with) -- screen blend mode treats black as a
     no-op, so it disappears into --player-bg (near-black) instead of
     showing as a visible box. */
  mix-blend-mode: screen;
}

.visual-static {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #888;
}

.info {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
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
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.title {
  font-weight: 600;
  font-size: 0.76rem;
}

.artist {
  font-size: 0.68rem;
  color: #a0a0a0;
}

.title:hover,
.artist:hover {
  text-decoration: underline;
}

.muted {
  font-size: 0.75rem;
  color: #a0a0a0;
}

.icon-btn {
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: none;
  border: none;
  color: #a0a0a0;
  cursor: pointer;
  padding: 0.25rem;
  border-radius: 4px;
}

.icon-btn:hover:not(:disabled) {
  color: white;
  background: rgba(255, 255, 255, 0.1);
}

.icon-btn.active {
  color: var(--accent);
}

.icon-btn:disabled {
  opacity: 0.35;
  cursor: default;
}

.expand-btn {
  color: #c8c8c8;
}

.progress-row {
  width: 100%;
  flex-shrink: 0;
}

/* Two stacked copies of the exact same bar shapes -- .wave (dim, full
   width) underneath, .wave.overlay (accent-colored) on top with a
   clip-path that reveals more of it as playback progresses. Both layers
   share identical layout, so the reveal happens at the pixel level
   (including mid-bar) instead of the accent color jumping bar-by-bar. */
.wave-stack {
  position: relative;
  width: 100%;
  height: 18px;
}

.wave-stack.seekable {
  cursor: pointer;
}

.wave {
  position: absolute;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 2px;
}

.wave.overlay span {
  background: var(--accent);
}

.wave span {
  flex: 1;
  min-width: 1.5px;
  border-radius: 1px;
  background: rgba(255, 255, 255, 0.16);
  transition: height 0.15s linear;
  pointer-events: none;
}

/* Decorative motion layered on top of the real per-track shape (not a
   substitute for it) -- off while paused and skipped entirely in
   Performance Mode. Deliberately synchronized (no per-bar stagger) and
   subtle: a staggered/bigger bounce puts bars at different points in the
   cycle at any given instant, which visibly washes out the real relative
   heights (a tall bar mid-dip can end up shorter than a quiet bar
   mid-peak) -- this keeps every bar at the same phase, so the shape stays
   readable as "the song" and just breathes gently instead of obscuring it. */
.wave.animated span {
  animation: wave-pulse 1.6s ease-in-out infinite;
}

@keyframes wave-pulse {
  0%, 100% { transform: scaleY(0.96); }
  50% { transform: scaleY(1.04); }
}

.transport-row {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.65rem;
  flex: 1;
}

.transport-btn,
.play-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  cursor: pointer;
  border-radius: 50%;
  flex-shrink: 0;
}

.transport-btn {
  width: 28px;
  height: 28px;
  background: rgba(255, 255, 255, 0.07);
  color: #e8e8e8;
}

.transport-btn:hover:not(:disabled) {
  background: rgba(255, 255, 255, 0.16);
}

.play-btn {
  width: 38px;
  height: 38px;
  background: white;
  color: #111;
}

.play-btn:hover:not(:disabled) {
  background: #eee;
}

.transport-btn:disabled,
.play-btn:disabled {
  opacity: 0.4;
  cursor: default;
}

.play-btn:disabled {
  background: #555;
  color: #999;
}

.toggle-btn {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
  background: none;
  border: none;
  cursor: pointer;
  color: #999;
  padding: 0.25rem;
  flex-shrink: 0;
}

.toggle-btn:hover {
  color: #ccc;
}

.toggle-btn.active {
  color: var(--accent);
}

.loop-one-badge {
  position: absolute;
  top: -1px;
  right: -1px;
  font-size: 0.5rem;
  font-weight: 700;
  line-height: 1;
  background: var(--accent);
  color: white;
  border-radius: 50%;
  width: 9px;
  height: 9px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.secondary-row {
  display: flex;
  align-items: center;
  gap: 0.15rem;
  flex-shrink: 0;
}

.row-spacer {
  flex: 1;
}

.mini-volume {
  width: 56px;
  accent-color: var(--accent);
}

.queue-view {
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
}

.queue-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  font-size: 0.75rem;
  font-weight: 600;
  color: #ccc;
  padding-bottom: 0.3rem;
  flex-shrink: 0;
}

.queue-empty {
  margin: 0;
  color: #888;
  font-size: 0.75rem;
}

.queue-view ul {
  list-style: none;
  margin: 0;
  padding: 0;
  overflow-y: auto;
  flex: 1;
  min-height: 0;
}

.queue-view li {
  display: flex;
  align-items: center;
  gap: 0.4rem;
  padding: 0.25rem 0.3rem;
  font-size: 0.72rem;
  cursor: pointer;
  border-radius: 4px;
}

.queue-view li:hover {
  background: rgba(255, 255, 255, 0.08);
}

.qtitle {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  max-width: 8rem;
}

.qartist {
  color: #888;
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>
