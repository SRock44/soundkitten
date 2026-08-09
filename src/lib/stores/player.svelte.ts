import { streamUrl } from "../api";
import { isPlayable, type Track } from "../types";

const VOLUME_KEY = "sc-desktop:volume";
const SHUFFLE_KEY = "sc-desktop:shuffle";
const LOOP_KEY = "sc-desktop:loop";

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
  /** Transient "skipped a DRM/unplayable track" message -- distinct from `error`, which is reserved for an actually-stuck player. Cleared once the next track starts playing. */
  notice = $state<string | null>(null);
  history = $state<Track[]>([]);
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

  get current(): Track | null {
    return this.queueIndex >= 0 && this.queueIndex < this.queue.length ? this.queue[this.queueIndex] : null;
  }

  get upcoming(): Track[] {
    return this.queue.slice(this.queueIndex + 1);
  }

  attach(el: HTMLAudioElement) {
    this.audioEl = el;
    el.volume = this.volume;
  }

  setVolume(v: number) {
    this.volume = v;
    if (this.audioEl) this.audioEl.volume = v;
    safeStorageSet(VOLUME_KEY, String(v));
  }

  toggleShuffle() {
    this.shuffle = !this.shuffle;
    this.shuffleFrontier = this.queueIndex;
    safeStorageSet(SHUFFLE_KEY, String(this.shuffle));
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
    const list = context.length ? context : [track];
    const idx = list.findIndex((t) => t.id === track.id);
    this.queue = list;
    this.queueIndex = idx >= 0 ? idx : 0;
    this.shuffleFrontier = this.queueIndex;
    this._loadCurrent(1, 0);
    // pushed after _loadCurrent so that if `track` itself turns out to be
    // unplayable and gets auto-skipped, we record what actually started
    // playing rather than the track the user clicked.
    this._pushHistory(this.current);
  }

  /** Insert `track` immediately after the currently playing one. */
  playNext(track: Track) {
    const insertAt = this.queueIndex < 0 ? 0 : this.queueIndex + 1;
    this.queue = [...this.queue.slice(0, insertAt), track, ...this.queue.slice(insertAt)];
    if (this.queueIndex < 0) {
      this.queueIndex = 0;
      this._loadCurrent();
    }
  }

  /** Append `track` to the end of the queue. */
  addToQueue(track: Track) {
    this.queue = [...this.queue, track];
    if (this.queueIndex < 0) {
      this.queueIndex = 0;
      this._loadCurrent();
    }
  }

  removeFromQueue(index: number) {
    if (index === this.queueIndex) return; // can't remove the currently-playing track this way
    this.queue = this.queue.filter((_, i) => i !== index);
    if (index < this.queueIndex) this.queueIndex -= 1;
  }

  /** Moves the track at `from` to `to` (only meaningful for upcoming, not-yet-played tracks). */
  reorderQueue(from: number, to: number) {
    if (from === this.queueIndex || to === this.queueIndex) return;
    if (from < 0 || from >= this.queue.length || to < 0 || to >= this.queue.length) return;
    const next = [...this.queue];
    const [moved] = next.splice(from, 1);
    next.splice(to, 0, moved);
    this.queue = next;
    // queueIndex only shifts if the currently-playing track's position moved
    // relative to it (its own index never changes since we forbid moving it).
    if (from < this.queueIndex && to >= this.queueIndex) this.queueIndex -= 1;
    else if (from > this.queueIndex && to <= this.queueIndex) this.queueIndex += 1;
  }

  /** Safety valve against skipping through an entire all-DRM queue forever. */
  private static readonly MAX_AUTO_SKIPS = 25;

  /**
   * Fetches the resolved audio ourselves (rather than pointing <audio src>
   * straight at sc-stream://) so that a failure surfaces our backend's real
   * error text (e.g. "DRM-protected", "no transcodings") instead of the
   * browser's generic, undiagnosable "no supported source" MediaError.
   *
   * `direction`/`skipDepth` thread through an unplayable-track auto-skip
   * chain (see `_skipUnplayable`) -- when the track we land on turns out to
   * be DRM-locked or otherwise fails to load, we advance one more step in
   * the same direction and try again, rather than leaving playback stuck.
   */
  private async _loadCurrent(direction: 1 | -1 = 1, skipDepth = 0) {
    if (!this.audioEl || !this.current) return;
    const track = this.current;
    const token = ++this.loadToken;
    this.error = null;

    // Checked up front (mirrors the backend's DRM detection) so a known-DRM
    // track never even round-trips through fetch before being skipped.
    if (!isPlayable(track)) {
      this._skipUnplayable(track, direction, skipDepth, "protected by SoundCloud DRM");
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
      this._skipUnplayable(track, direction, skipDepth, message || `HTTP ${resp.status}`);
      return;
    }

    const blob = await resp.blob();
    if (token !== this.loadToken) return;

    if (this.currentBlobUrl) URL.revokeObjectURL(this.currentBlobUrl);
    this.currentBlobUrl = URL.createObjectURL(blob);
    this.audioEl.src = this.currentBlobUrl;
    if (skipDepth > 0) this.notice = null; // successfully recovered from a skip chain
    this.audioEl.play().catch((e) => {
      if (token === this.loadToken) this.error = e instanceof Error ? e.message : String(e);
    });
  }

  /**
   * A track failed to load -- rather than leaving playback frozen on it,
   * silently step to the next (or previous) track in the same direction and
   * try that one instead. Bounded by MAX_AUTO_SKIPS so a queue that's all
   * unplayable doesn't recurse forever.
   */
  private _skipUnplayable(track: Track, direction: 1 | -1, skipDepth: number, detail: string) {
    this.notice = `Skipped "${track.title ?? "track"}" (${detail}).`;

    if (skipDepth + 1 >= PlayerStore.MAX_AUTO_SKIPS) {
      this.error = "Too many unplayable tracks in a row -- stopped.";
      this.audioEl?.pause();
      return;
    }
    if (!this._stepIndex(direction)) {
      this.error = `"${track.title ?? "track"}" is unavailable and there's nothing else to play.`;
      this.audioEl?.pause(); // nothing left to skip to in this direction
      return;
    }
    this._loadCurrent(direction, skipDepth + 1);
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

  toggle() {
    if (!this.audioEl || !this.current) return;
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
    if (!this._stepIndex(1)) return;
    this._loadCurrent(1, 0);
    this._pushHistory(this.current);
  }

  previous() {
    if (!this._stepIndex(-1)) return;
    this._loadCurrent(-1, 0);
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
