import { streamUrl } from "../api";
import type { Track } from "../types";

const VOLUME_KEY = "sc-desktop:volume";

class PlayerStore {
  queue = $state<Track[]>([]);
  queueIndex = $state(-1);
  isPlaying = $state(false);
  error = $state<string | null>(null);
  history = $state<Track[]>([]);
  volume = $state(loadVolume());
  audioEl: HTMLAudioElement | null = null;
  private currentBlobUrl: string | null = null;
  private loadToken = 0;

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
    localStorage.setItem(VOLUME_KEY, String(v));
  }

  /** Replace the queue with `context` (or just `track`) and play `track` immediately. */
  play(track: Track, context: Track[] = []) {
    if (!this.audioEl) return;
    const list = context.length ? context : [track];
    const idx = list.findIndex((t) => t.id === track.id);
    this.queue = list;
    this.queueIndex = idx >= 0 ? idx : 0;
    this._loadCurrent();
    this.history = [track, ...this.history.filter((t) => t.id !== track.id)].slice(0, 20);
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

  /**
   * Fetches the resolved audio ourselves (rather than pointing <audio src>
   * straight at sc-stream://) so that a failure surfaces our backend's real
   * error text (e.g. "DRM-protected", "no transcodings") instead of the
   * browser's generic, undiagnosable "no supported source" MediaError.
   */
  private async _loadCurrent() {
    if (!this.audioEl || !this.current) return;
    const track = this.current;
    const token = ++this.loadToken;
    this.error = null;

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
      this.error = message || `Failed to load stream (HTTP ${resp.status})`;
      return;
    }

    const blob = await resp.blob();
    if (token !== this.loadToken) return;

    if (this.currentBlobUrl) URL.revokeObjectURL(this.currentBlobUrl);
    this.currentBlobUrl = URL.createObjectURL(blob);
    this.audioEl.src = this.currentBlobUrl;
    this.audioEl.play().catch((e) => {
      if (token === this.loadToken) this.error = e instanceof Error ? e.message : String(e);
    });
  }

  toggle() {
    if (!this.audioEl || !this.current) return;
    if (this.audioEl.paused) {
      this.audioEl.play().catch((e) => (this.error = e instanceof Error ? e.message : String(e)));
    } else {
      this.audioEl.pause();
    }
  }

  next() {
    if (this.queueIndex + 1 >= this.queue.length) return;
    this.queueIndex += 1;
    this._loadCurrent();
    const t = this.current;
    if (t) this.history = [t, ...this.history.filter((h) => h.id !== t.id)].slice(0, 20);
  }

  previous() {
    if (this.queueIndex <= 0) return;
    this.queueIndex -= 1;
    this._loadCurrent();
  }
}

function loadVolume(): number {
  if (typeof localStorage === "undefined") return 1;
  const stored = localStorage.getItem(VOLUME_KEY);
  const v = stored !== null ? Number(stored) : 1;
  return Number.isFinite(v) ? Math.min(1, Math.max(0, v)) : 1;
}

export const player = new PlayerStore();
