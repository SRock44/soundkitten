import { api, streamUrl } from "../api";
import { isPlayable, type Track } from "../types";

const VOLUME_KEY = "sc-desktop:volume";
const SHUFFLE_KEY = "sc-desktop:shuffle";
const LOOP_KEY = "sc-desktop:loop";
const PLAYBACK_KEY = "sc-desktop:playback";

/**
 * Guards every localStorage access. Not just defensive theater: Node's
 * built-in `localStorage` global (no backing file configured) throws on
 * every call, and browsers can throw in private-browsing modes too -- this
 * keeps persistence a nice-to-have rather than something that can crash
 * playback.
 */
function safeStorageGet(key: string): string | null {
  try {
    return localStorage.getItem(key);
  } catch {
    return null;
  }
}

function safeStorageSet(key: string, value: string) {
  try {
    localStorage.setItem(key, value);
  } catch {
    // ignore
  }
}

export type LoopMode = "off" | "all" | "one";
const LOOP_MODES: LoopMode[] = ["off", "all", "one"];

export class PlayerStore {
  queue = $state<Track[]>([]);
  queueIndex = $state(-1);
  isPlaying = $state(false);
  error = $state<string | null>(null);
  /** Transient status message ("skipped a DRM/unplayable track", "queue ended, playing related tracks") -- distinct from `error`, which is reserved for an actually-stuck player. Always shown through _showNotice() below, which auto-clears it after a few seconds so it never lingers as a permanent banner. */
  notice = $state<string | null>(null);
  private noticeTimer: ReturnType<typeof setTimeout> | undefined;
  history = $state<Track[]>([]);
  /** Set by whoever started the current queue from a playlist/mix (see
   * PlaylistCard.svelte), so its card can show pause instead of play and
   * toggle in place instead of re-fetching. `play()` resets this
   * unconditionally, since it's the single entry point every "start
   * something fresh" call goes through (individual track clicks included)
   * -- only the playlist-originated caller re-sets it right after. */
  currentPlaylistKey = $state<string | null>(null);
  volume = $state(loadVolume());
  shuffle = $state(loadShuffle());
  loop = $state<LoopMode>(loadLoop());
  audioEl: HTMLAudioElement | null = null;
  private currentBlobUrl: string | null = null;
  private loadToken = 0;
  /** Overridable for deterministic tests -- defaults to Math.random. */
  random: () => number = Math.random;
  /**
   * The highest queue index whose track has already been fixed by a shuffle
   * swap (or was never shuffled). Without this, next() would re-randomize
   * the same upcoming slot every time it's called, so next() -> previous()
   * -> next() would NOT replay the track you just heard -- it'd reroll a
   * fresh one, which breaks the everyday "go back and forward" case.
   */
  private shuffleFrontier = -1;
  private lastPersistAt = 0;
  /**
   * The queue in its real, unshuffled context order -- kept in sync with
   * every structural change (play/addToQueue/playNext/removeFromQueue), but
   * NOT with shuffled-view reordering. Turning shuffle off rebuilds `queue`
   * from this, which is the piece that was previously missing entirely:
   * `queue` was shuffled in place with no backup, so disabling shuffle had
   * nothing to restore from and just kept whatever shuffled order was there.
   */
  private originalQueue: Track[] = [];
  /**
   * A restored-but-not-yet-loaded position, set by restore() and consumed by
   * the first toggle()/play. Exposed (not private) so the UI can show the
   * right scrubber position immediately without that requiring a network
   * fetch -- see restore()'s comment for why loading is deferred at all.
   */
  pendingSeek = $state<number | null>(null);
  /** Raw position/duration from the <audio> element -- see onTimeUpdate/onDurationChange. Private: read via the currentTime/duration getters below, which also account for pendingSeek. */
  private _liveCurrentTime = $state(0);
  private _liveDuration = $state(0);
  /** Wall-clock time of the last onTimeUpdate() call -- see the watchdog in startPositionWatchdog(). */
  private lastTimeUpdateAt = 0;
  private watchdogId: ReturnType<typeof setInterval> | null = null;

  get current(): Track | null {
    return this.queueIndex >= 0 && this.queueIndex < this.queue.length ? this.queue[this.queueIndex] : null;
  }

  get upcoming(): Track[] {
    return this.queue.slice(this.queueIndex + 1);
  }

  /** Live playback position in seconds -- pendingSeek (a restored-but-not-yet-loaded position) takes priority, same as before this was owned by the store. */
  get currentTime(): number {
    return this.pendingSeek !== null ? this.pendingSeek : this._liveCurrentTime;
  }

  /** Live track duration in seconds -- falls back to the track's own metadata (no network needed) until the real <audio> element has it. */
  get duration(): number {
    if (this._liveDuration > 0) return this._liveDuration;
    return (this.current?.duration ?? 0) / 1000;
  }

  attach(el: HTMLAudioElement) {
    this.audioEl = el;
    el.volume = this.volume;
    this.startPositionWatchdog();
  }

  /** Wire to <audio>'s ontimeupdate. */
  onTimeUpdate() {
    if (!this.audioEl) return;
    this._liveCurrentTime = this.audioEl.currentTime;
    this.lastTimeUpdateAt = Date.now();
    this.savePositionTick();
  }

  /** Wire to <audio>'s ondurationchange. */
  onDurationChange() {
    if (!this.audioEl) return;
    this._liveDuration = this.audioEl.duration || 0;
  }

  /**
   * Seeks playback and updates `currentTime` immediately -- doesn't wait for
   * the next timeupdate/watchdog tick, which (if paused, or mid-flight)
   * might not land for a while and would otherwise leave the scrubber
   * showing the pre-seek position.
   */
  seek(target: number) {
    if (!this.audioEl || !this.duration) return;
    if (this.pendingSeek !== null && !this.audioEl.src) {
      this.loadPendingRestore(target);
      return;
    }
    this.audioEl.currentTime = target;
    this._liveCurrentTime = target;
    this.lastTimeUpdateAt = Date.now();
  }

  /**
   * `ontimeupdate` is not a reliable heartbeat -- confirmed live that a
   * flood of rapid, unrelated UI events (e.g. dragging the volume slider,
   * which fires dozens of `input` events a second) can starve the event
   * loop long enough that it silently stops arriving for a stretch, while
   * the real audio keeps playing underneath regardless. Rather than an
   * unconditional poll (which was tried and reverted -- it raced with
   * seek(): reading the element back on a fixed timer could catch it before
   * a fresh seek had settled and stomp the just-set position with a stale
   * read), this only steps in once onTimeUpdate() has gone quiet for
   * nearly a full second despite playback supposedly being active --  well
   * past the point any seek's synchronous currentTime write would have
   * taken effect, so it never has a fresh seek to clobber.
   */
  private startPositionWatchdog() {
    if (this.watchdogId !== null) return;
    this.watchdogId = setInterval(() => {
      if (!this.isPlaying || !this.audioEl) return;
      if (Date.now() - this.lastTimeUpdateAt < 900) return;
      this.onTimeUpdate();
    }, 1000);
  }

  /** Stops the watchdog interval -- call on teardown (component unmount / test cleanup) so it doesn't keep firing against a torn-down audio element. */
  destroy() {
    if (this.watchdogId !== null) {
      clearInterval(this.watchdogId);
      this.watchdogId = null;
    }
    clearTimeout(this.noticeTimer);
  }

  /**
   * Shows a transient status message and schedules it to clear itself --
   * these are meant to be a passing "here's what just happened", not a
   * permanent banner. Previously "Queue ended -- now playing related
   * tracks" had no clear path at all (unlike the DRM-skip notice below,
   * which gets cleared early by a successful recovery load) and just sat
   * there indefinitely once shown.
   */
  private _showNotice(text: string, ms = 6000) {
    clearTimeout(this.noticeTimer);
    this.notice = text;
    this.noticeTimer = setTimeout(() => this._clearNotice(), ms);
  }

  private _clearNotice() {
    clearTimeout(this.noticeTimer);
    this.noticeTimer = undefined;
    this.notice = null;
  }

  /**
   * Restores the queue and "where you left off" position from the last app
   * run, mirroring soundcloud.com's own behavior -- but deliberately does
   * NOT fetch/stream anything yet. It used to call _loadCurrent() right
   * away, which meant every single cold launch immediately resolved and
   * streamed a track's audio before the user had touched anything -- one
   * more request in the exact burst (alongside likes/playlists/me/...) that
   * was getting the account rate-limited. Now it only sets state (instant,
   * no network), and the real load happens lazily on the first toggle()
   * once the user actually presses play.
   */
  async restore() {
    if (!this.audioEl) return;
    const raw = safeStorageGet(PLAYBACK_KEY);
    if (!raw) return;
    let saved: { queue: Track[]; originalQueue?: Track[]; queueIndex: number; positionSeconds: number } | null = null;
    try {
      saved = JSON.parse(raw);
    } catch {
      return;
    }
    if (!saved || !Array.isArray(saved.queue) || saved.queue.length === 0) return;
    if (saved.queueIndex < 0 || saved.queueIndex >= saved.queue.length) return;

    this.queue = saved.queue;
    // Older saves (or a corrupted/partial one) may lack this -- fall back to
    // the shuffled queue itself rather than leaving shuffle-off with nothing
    // to restore to.
    this.originalQueue = Array.isArray(saved.originalQueue) && saved.originalQueue.length > 0 ? saved.originalQueue : saved.queue;
    this.queueIndex = saved.queueIndex;
    // If shuffle was already on when this got saved, `saved.queue` IS the
    // real, already-fully-shuffled order (persisted straight from `queue`)
    // -- marking it as "unfixed" (the old behavior) meant the very next
    // next() after every app restart would run _shuffleNextSlot() and
    // silently swap in a *different* track than whatever the "Up next"
    // panel had just shown, since that guard only treats indices <=
    // shuffleFrontier as already settled. `this.shuffle` reflects the
    // persisted setting already (loaded in the field initializer above,
    // before restore() ever runs), so this is safe to check here.
    this.shuffleFrontier = this.shuffle ? this.queue.length - 1 : saved.queueIndex;
    this.pendingSeek = saved.positionSeconds > 0 ? saved.positionSeconds : null;
  }

  /** Immediate (non-throttled) persist -- call after any queue/queueIndex mutation. */
  private _persistQueue() {
    if (this.queue.length === 0) {
      safeStorageSet(PLAYBACK_KEY, "");
      return;
    }
    const positionSeconds = this.audioEl?.currentTime ?? 0;
    safeStorageSet(
      PLAYBACK_KEY,
      JSON.stringify({ queue: this.queue, originalQueue: this.originalQueue, queueIndex: this.queueIndex, positionSeconds }),
    );
  }

  /**
   * Position-only persist meant to be called from <audio>'s frequent
   * ontimeupdate -- throttled to at most once every 5s so playback progress
   * doesn't hammer localStorage on every tick. Pass `force` for natural
   * checkpoints like pause, where a fresher save is worth the one-off cost.
   */
  savePositionTick(force = false) {
    const now = Date.now();
    if (!force && now - this.lastPersistAt < 5000) return;
    this.lastPersistAt = now;
    this._persistQueue();
  }

  clearPersistedPlayback() {
    safeStorageSet(PLAYBACK_KEY, "");
  }

  setVolume(v: number) {
    this.volume = v;
    if (this.audioEl) this.audioEl.volume = v;
    safeStorageSet(VOLUME_KEY, String(v));
  }

  private preMuteVolume = 1;

  toggleMute() {
    if (this.volume > 0) {
      this.preMuteVolume = this.volume;
      this.setVolume(0);
    } else {
      this.setVolume(this.preMuteVolume > 0 ? this.preMuteVolume : 1);
    }
  }

  toggleShuffle() {
    this.shuffle = !this.shuffle;
    safeStorageSet(SHUFFLE_KEY, String(this.shuffle));
    // Shuffle the whole remaining queue immediately (rather than only the
    // lazy "next slot" _shuffleNextSlot does as you advance) so the "Up
    // next" panel actually reflects shuffled order right away instead of
    // still showing the original sequence until you've pressed Next once.
    if (this.shuffle) this._shuffleRemainingNow();
    else this._restoreOriginalOrder();
    this._persistQueue();
  }

  /** Rebuilds `queue` from `originalQueue`, keeping the currently-playing track as current. */
  private _restoreOriginalOrder() {
    const current = this.current;
    this.queue = [...this.originalQueue];
    const idx = current ? this.originalQueue.findIndex((t) => t.id === current.id) : -1;
    this.queueIndex = idx >= 0 ? idx : Math.min(this.queueIndex, this.queue.length - 1);
    this.shuffleFrontier = this.queueIndex;
  }

  /** Cycles off -> all -> one -> off. */
  cycleLoop() {
    const i = LOOP_MODES.indexOf(this.loop);
    this.loop = LOOP_MODES[(i + 1) % LOOP_MODES.length];
    safeStorageSet(LOOP_KEY, this.loop);
  }

  /** Replace the queue with `context` (or just `track`) and play `track` immediately. */
  play(track: Track, context: Track[] = []) {
    if (!this.audioEl) return;
    this.pendingSeek = null; // starting a fresh context invalidates any restored-but-unloaded position
    this.currentPlaylistKey = null;
    const list = context.length ? context : [track];
    const idx = list.findIndex((t) => t.id === track.id);
    this.queue = list;
    this.originalQueue = list; // new context -- this is the new unshuffled baseline
    this.queueIndex = idx >= 0 ? idx : 0;
    this.shuffleFrontier = this.queueIndex;
    if (this.shuffle) this._shuffleRemainingNow();
    this._loadCurrent(1, 0);
    // pushed after _loadCurrent so that if `track` itself turns out to be
    // unplayable and gets auto-skipped, we record what actually started
    // playing rather than the track the user clicked.
    this._pushHistory(this.current);
    this._persistQueue();
  }

  /** Insert `track` immediately after the currently playing one. */
  playNext(track: Track) {
    const insertAt = this.queueIndex < 0 ? 0 : this.queueIndex + 1;
    this.queue = [...this.queue.slice(0, insertAt), track, ...this.queue.slice(insertAt)];
    const origInsertAt = this._origIndexOfCurrent() + 1;
    this.originalQueue = [...this.originalQueue.slice(0, origInsertAt), track, ...this.originalQueue.slice(origInsertAt)];
    // Everything at/after insertAt just shifted right by one -- shuffleFrontier
    // is an index into that same range, so it has to shift too or it'll end
    // up pointing one slot short of the track it used to mark as "already
    // fixed", letting _shuffleNextSlot() re-randomize a track that was
    // already settled (and already shown as such in the queue panel).
    if (insertAt <= this.shuffleFrontier) this.shuffleFrontier += 1;
    if (this.queueIndex < 0) {
      this.queueIndex = 0;
      this._loadCurrent();
    }
    this._persistQueue();
  }

  /** Append `track` to the end of the queue. */
  addToQueue(track: Track) {
    this.queue = [...this.queue, track];
    this.originalQueue = [...this.originalQueue, track];
    if (this.queueIndex < 0) {
      this.queueIndex = 0;
      this._loadCurrent();
    }
    this._persistQueue();
  }

  removeFromQueue(index: number) {
    if (index === this.queueIndex) return; // can't remove the currently-playing track this way
    const removed = this.queue[index];
    this.queue = this.queue.filter((_, i) => i !== index);
    if (removed) this.originalQueue = this.originalQueue.filter((t) => t.id !== removed.id);
    if (index < this.queueIndex) this.queueIndex -= 1;
    if (index <= this.shuffleFrontier) this.shuffleFrontier -= 1; // mirror image of the playNext() shift above
    this._persistQueue();
  }

  /**
   * Jumps straight to `index` in the queue and starts playing it -- used by
   * the "Up next" panel's click-to-play. Just moves the play head; doesn't
   * reorder anything, so it works the same whether shuffle is on or off.
   */
  playFromQueue(index: number) {
    if (!this.audioEl || index < 0 || index >= this.queue.length || index === this.queueIndex) return;
    this.pendingSeek = null;
    this.queueIndex = index;
    // Jumping ahead of the fixed range means everything up to here is now
    // "settled" too -- same as if the user had pressed next() that many
    // times -- so a later next() doesn't turn around and re-shuffle a track
    // that was just sitting there, visibly picked, in the queue panel.
    if (this.shuffle && index > this.shuffleFrontier) this.shuffleFrontier = index;
    this._loadCurrent(1, 0);
    this._pushHistory(this.current);
    this._persistQueue();
  }

  /** Index of the currently-playing track within `originalQueue` (falls back to the end). */
  private _origIndexOfCurrent(): number {
    const current = this.current;
    if (!current) return this.originalQueue.length - 1;
    const idx = this.originalQueue.findIndex((t) => t.id === current.id);
    return idx >= 0 ? idx : this.originalQueue.length - 1;
  }

  /** Moves the track at `from` to `to` (only meaningful for upcoming, not-yet-played tracks). */
  reorderQueue(from: number, to: number) {
    if (from === this.queueIndex || to === this.queueIndex) return;
    if (from < 0 || from >= this.queue.length || to < 0 || to >= this.queue.length) return;
    const next = [...this.queue];
    const [moved] = next.splice(from, 1);
    next.splice(to, 0, moved);
    // Only mirrored into the unshuffled baseline when not currently
    // shuffled -- dragging within a shuffled view is a temporary rearrange
    // of that view, not a redefinition of "what order this was in" for when
    // shuffle gets turned back off.
    if (!this.shuffle) {
      const origNext = [...this.originalQueue];
      const [movedOrig] = origNext.splice(from, 1);
      origNext.splice(to, 0, movedOrig);
      this.originalQueue = origNext;
    }
    this.queue = next;
    // queueIndex only shifts if the currently-playing track's position moved
    // relative to it (its own index never changes since we forbid moving it).
    if (from < this.queueIndex && to >= this.queueIndex) this.queueIndex -= 1;
    else if (from > this.queueIndex && to <= this.queueIndex) this.queueIndex += 1;
    this._persistQueue();
  }

  /** Safety valve against skipping through an entire all-DRM queue forever. */
  private static readonly MAX_AUTO_SKIPS = 25;

  /**
   * Fetches the resolved audio ourselves (rather than pointing <audio src>
   * straight at sc-stream://) so that a failure surfaces our backend's real
   * error text (e.g. "DRM-protected", "no transcodings") instead of the
   * browser's generic, undiagnosable "no supported source" MediaError.
   *
   * This does mean a whole track's compressed bytes sit in a JS Blob for as
   * long as it's playing -- worth knowing, but not worth "fixing" by having
   * <audio> issue its own separate request instead: the sc-stream:// handler
   * (src-tauri/src/playback.rs) fully downloads/assembles a track server-side
   * before responding at all (no streaming response), so a second fetch
   * would re-run that entire download against SoundCloud's CDN rather than
   * saving anything -- strictly worse for both bandwidth and latency.
   *
   * `direction`/`skipDepth` thread through an unplayable-track auto-skip
   * chain (see `_skipUnplayable`) -- when the track we land on turns out to
   * be DRM-locked or otherwise fails to load, we advance one more step in
   * the same direction and try again, rather than leaving playback stuck.
   */
  private async _loadCurrent(direction: 1 | -1 = 1, skipDepth = 0, opts: { autoplay?: boolean; seekTo?: number } = {}) {
    const autoplay = opts.autoplay ?? true;
    if (!this.audioEl || !this.current) return;
    const track = this.current;
    const token = ++this.loadToken;
    this.error = null;

    // Checked up front (mirrors the backend's DRM detection) so a known-DRM
    // track never even round-trips through fetch before being skipped.
    if (!isPlayable(track)) {
      this._skipUnplayable(track, direction, skipDepth, "protected by SoundCloud DRM", { autoplay });
      return;
    }

    let resp: Response;
    try {
      resp = await fetch(streamUrl(track.id));
    } catch (e) {
      if (token === this.loadToken) this.error = e instanceof Error ? e.message : String(e);
      return;
    }
    if (token !== this.loadToken) return; // a newer track was requested while this was in flight

    // The backend always answers 200 (a WebView2 quirk turns non-2xx custom-
    // protocol responses into an opaque "Failed to fetch" instead of a
    // readable error), so real failures are signaled via this header instead.
    const errorHeader = resp.headers.get("X-Sc-Error");
    if (errorHeader || !resp.ok) {
      let message = "";
      try {
        message = errorHeader ? atob(errorHeader) : "";
      } catch {
        message = errorHeader ?? "";
      }
      this._skipUnplayable(track, direction, skipDepth, message || `HTTP ${resp.status}`, { autoplay });
      return;
    }

    const blob = await resp.blob();
    if (token !== this.loadToken) return;

    if (this.currentBlobUrl) URL.revokeObjectURL(this.currentBlobUrl);
    this.currentBlobUrl = URL.createObjectURL(blob);
    this.audioEl.src = this.currentBlobUrl;
    if (skipDepth > 0) this._clearNotice(); // successfully recovered from a skip chain

    if (opts.seekTo !== undefined) {
      const el = this.audioEl;
      const seekTarget = opts.seekTo;
      const onLoaded = () => {
        el.currentTime = seekTarget;
        el.removeEventListener("loadedmetadata", onLoaded);
      };
      el.addEventListener("loadedmetadata", onLoaded);
    }

    if (autoplay) {
      this.audioEl.play().catch((e) => {
        if (token === this.loadToken) this.error = e instanceof Error ? e.message : String(e);
      });
    }
  }

  /**
   * A track failed to load -- rather than leaving playback frozen on it,
   * silently step to the next (or previous) track in the same direction and
   * try that one instead. Bounded by MAX_AUTO_SKIPS so a queue that's all
   * unplayable doesn't recurse forever.
   */
  private _skipUnplayable(
    track: Track,
    direction: 1 | -1,
    skipDepth: number,
    detail: string,
    opts: { autoplay?: boolean } = {},
  ) {
    this._showNotice(`Skipped "${track.title ?? "track"}" (${detail}).`);

    if (skipDepth + 1 >= PlayerStore.MAX_AUTO_SKIPS) {
      this.error = "Too many unplayable tracks in a row -- stopped.";
      this.audioEl?.pause();
      return;
    }
    if (!this._stepIndex(direction)) {
      // Reaching the end going forward is exactly the same "nowhere left to
      // go" case next() hits at the natural end of a queue -- fall through
      // to the same related-tracks extension instead of duplicating it here.
      if (direction === 1) {
        this._tryExtendWithRelated(() => {
          this.error = `"${track.title ?? "track"}" is unavailable and there's nothing else to play.`;
          this.audioEl?.pause();
        });
        return;
      }
      this.error = `"${track.title ?? "track"}" is unavailable and there's nothing else to play.`;
      this.audioEl?.pause(); // nothing left to skip to in this direction
      return;
    }
    // seekTo intentionally dropped -- a saved position only applies to the
    // original restored track, not whatever we land on after skipping past it.
    this._loadCurrent(direction, skipDepth + 1, opts);
  }

  /**
   * Moves `queueIndex` one step in `direction`, applying shuffle-as-you-go
   * (forward only) and loop=all wraparound. Returns false if there's nowhere
   * left to go. Shared by next()/previous() and the auto-skip chain so a
   * skip never has to re-enter the public next()/previous() API (which
   * would each push their own, now-stale, history entry mid-chain).
   */
  private _stepIndex(direction: 1 | -1): boolean {
    if (direction === 1) {
      if (this.shuffle) this._shuffleNextSlot();
      if (this.queueIndex + 1 >= this.queue.length) {
        if (this.loop === "all" && this.queue.length > 0) {
          this.queueIndex = 0;
          return true;
        }
        return false;
      }
      this.queueIndex += 1;
      return true;
    }
    if (this.queueIndex <= 0) {
      if (this.loop === "all" && this.queue.length > 0) {
        this.queueIndex = this.queue.length - 1;
        return true;
      }
      return false;
    }
    this.queueIndex -= 1;
    return true;
  }

  /**
   * Actually loads a restored-but-deferred track (see restore()), seeking to
   * `seconds` if given or the originally-saved position otherwise. Exposed
   * publicly so any first interaction -- pressing play, or clicking the
   * scrubber to a specific spot -- can trigger it, not just play.
   */
  loadPendingRestore(seconds?: number) {
    if (this.pendingSeek === null || !this.audioEl || this.audioEl.src) return;
    const seekTo = seconds ?? this.pendingSeek;
    this.pendingSeek = null;
    this._loadCurrent(1, 0, { seekTo });
  }

  toggle() {
    if (!this.audioEl || !this.current) return;
    if (this.pendingSeek !== null && !this.audioEl.src) {
      this.loadPendingRestore();
      return;
    }
    if (this.audioEl.paused) {
      this.audioEl.play().catch((e) => (this.error = e instanceof Error ? e.message : String(e)));
    } else {
      this.audioEl.pause();
    }
  }

  /**
   * Advances to the next track. When shuffle is on, picks a random track
   * from the not-yet-played remainder of the queue and swaps it into the
   * next slot ("shuffle as you go") -- this keeps `queue` as the single
   * source of truth (so playNext/addToQueue/removeFromQueue/reorderQueue
   * don't need shuffle-aware duplicate bookkeeping) and makes `previous()`
   * work unmodified, since the shuffled order is now just the array order.
   *
   * If the landed-on track is DRM-locked or otherwise fails to load,
   * _loadCurrent transparently keeps skipping forward on our behalf instead
   * of leaving playback stuck, so `this.current` below reflects wherever
   * that chain actually settled.
   */
  next() {
    this.pendingSeek = null; // moving off the restored track invalidates its pending seek
    if (!this._stepIndex(1)) {
      this._tryExtendWithRelated();
      return;
    }
    this._loadCurrent(1, 0);
    this._pushHistory(this.current);
    this._persistQueue();
  }

  /**
   * Called whenever there's nowhere left to advance to (queue exhausted,
   * loop isn't "all") -- rather than just going silent, pulls in tracks
   * related to whatever just finished and keeps playing, the same "keep
   * listening" behavior SoundCloud's own app has at the end of a playlist,
   * album, or artist queue. Appends to the existing queue (so it's still
   * exactly the original playlist/album with more tacked on, not a
   * replacement) and advances into the first new track.
   *
   * Gives up quietly by default -- playback just stops, as it did before
   * this existed -- if the fetch fails or comes back with nothing genuinely
   * new (e.g. everything returned is already in this queue). That's the
   * right outcome for the everyday "you reached the end of your playlist"
   * case: not an error, just nothing more to play. `onNoneFound`, passed
   * only from the DRM-skip-chain call site, restores the real error message
   * for the genuinely-different case of a queue that turned out to be
   * entirely unplayable -- there this IS a failure worth surfacing, not a
   * graceful end.
   */
  private async _tryExtendWithRelated(onNoneFound?: () => void) {
    const seed = this.current;
    if (!seed) {
      onNoneFound?.();
      return;
    }
    const tokenAtCallTime = this.loadToken;
    let related: Track[];
    try {
      related = await api.relatedTracks(seed.id);
    } catch {
      onNoneFound?.();
      return;
    }
    // A new load (manual track pick, a totally different play() call, etc.)
    // started while that fetch was in flight -- this result is stale, don't
    // act on it (and don't run onNoneFound either -- something else already
    // happened, this fetch just no longer matters).
    if (tokenAtCallTime !== this.loadToken) return;
    const existingIds = new Set(this.queue.map((t) => t.id));
    const fresh = related.filter((t) => !existingIds.has(t.id));
    if (fresh.length === 0) {
      onNoneFound?.();
      return;
    }

    this.queue = [...this.queue, ...fresh];
    this.originalQueue = [...this.originalQueue, ...fresh];
    // Shuffle the newly-appended tail immediately rather than leaving it to
    // the lazy per-slot shuffle -- otherwise the queue panel shows these
    // related tracks in SoundCloud's original recommendation order (looking
    // exactly like shuffle isn't applied at all) until you've stepped deep
    // enough into them for _shuffleNextSlot to have touched each one.
    if (this.shuffle) this._shuffleRemainingNow();
    else this.shuffleFrontier = this.queueIndex;
    this.queueIndex += 1;
    this._showNotice("Queue ended -- now playing related tracks");
    this._loadCurrent(1, 0);
    this._pushHistory(this.current);
    this._persistQueue();
  }

  /** Restarting counts as "meaningfully into the track" past this point -- matches the ~3s threshold most music players use for skip-back. */
  private static readonly RESTART_THRESHOLD_SECONDS = 3;

  /**
   * Standard music-player skip-back behavior: if the current track has
   * already played past a few seconds, pressing "previous" restarts it from
   * 0:00 instead of jumping to the actual previous track -- pressing it
   * again (now at/near 0:00) is what actually goes back. Matches Spotify/
   * Apple Music/etc, and means a stray double-tap near a track boundary
   * doesn't skip two tracks back.
   */
  previous() {
    if (this.current && this.currentTime > PlayerStore.RESTART_THRESHOLD_SECONDS) {
      this.seek(0);
      return;
    }
    this.pendingSeek = null;
    if (!this._stepIndex(-1)) return;
    this._loadCurrent(-1, 0);
    this._persistQueue();
  }

  /** Wire this to the <audio> element's `onended` event. */
  onTrackEnded() {
    if (this.loop === "one" && this.audioEl) {
      this.audioEl.currentTime = 0;
      this.audioEl.play().catch((e) => (this.error = e instanceof Error ? e.message : String(e)));
      return;
    }
    this.next();
  }

  private _shuffleNextSlot() {
    const remainingStart = this.queueIndex + 1;
    if (remainingStart <= this.shuffleFrontier) return; // already fixed by an earlier shuffle
    this.shuffleFrontier = remainingStart;

    const poolSize = this.queue.length - remainingStart;
    if (poolSize <= 1) return;
    const randomIndex = remainingStart + Math.floor(this.random() * poolSize);
    if (randomIndex === remainingStart) return;
    const next = [...this.queue];
    [next[remainingStart], next[randomIndex]] = [next[randomIndex], next[remainingStart]];
    this.queue = next;
  }

  /**
   * Fisher-Yates shuffle of everything after the current track, done all at
   * once (rather than lazily one slot at a time like _shuffleNextSlot) so
   * the queue array -- the same one the "Up next" UI reads from -- reflects
   * the real upcoming order immediately. Marks the whole shuffled range as
   * "fixed" via shuffleFrontier, so tracks added later still get randomized
   * into place lazily by _shuffleNextSlot when actually reached.
   */
  private _shuffleRemainingNow() {
    const start = this.queueIndex + 1;
    if (start >= this.queue.length) {
      this.shuffleFrontier = this.queueIndex;
      return;
    }
    const next = [...this.queue];
    for (let i = next.length - 1; i > start; i--) {
      const j = start + Math.floor(this.random() * (i - start + 1));
      [next[i], next[j]] = [next[j], next[i]];
    }
    this.queue = next;
    this.shuffleFrontier = next.length - 1;
  }

  private _pushHistory(track: Track | null) {
    if (!track) return;
    this.history = [track, ...this.history.filter((h) => h.id !== track.id)].slice(0, 20);
  }
}

function loadVolume(): number {
  if (typeof localStorage === "undefined") return 1;
  const stored = safeStorageGet(VOLUME_KEY);
  const v = stored !== null ? Number(stored) : 1;
  return Number.isFinite(v) ? Math.min(1, Math.max(0, v)) : 1;
}

function loadShuffle(): boolean {
  if (typeof localStorage === "undefined") return false;
  return safeStorageGet(SHUFFLE_KEY) === "true";
}

function loadLoop(): LoopMode {
  if (typeof localStorage === "undefined") return "off";
  const stored = safeStorageGet(LOOP_KEY);
  return stored === "all" || stored === "one" ? stored : "off";
}

export const player = new PlayerStore();
