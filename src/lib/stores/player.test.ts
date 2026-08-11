import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { PlayerStore } from "./player.svelte";
import { api } from "../api";
import type { Track } from "../types";

vi.mock("../api", () => ({
  api: { relatedTracks: vi.fn() },
  streamUrl: (id: number) => `sc-stream://${id}`,
}));

function makeTrack(id: number, overrides: Partial<Track> = {}): Track {
  return {
    id,
    title: `Track ${id}`,
    permalink_url: `https://soundcloud.com/artist/track-${id}`,
    artwork_url: null,
    duration: 180000,
    genre: null,
    user: null,
    streamable: true,
    likes_count: null,
    reposts_count: null,
    comment_count: null,
    playback_count: null,
    created_at: null,
    media: { transcodings: [{ format: { protocol: "progressive", mime_type: "audio/mpeg" } }] },
    waveform_url: null,
    ...overrides,
  };
}

function makeDrmTrack(id: number, overrides: Partial<Track> = {}): Track {
  return makeTrack(id, {
    media: { transcodings: [{ format: { protocol: "cbc-encrypted-hls", mime_type: "audio/mp4" } }] },
    ...overrides,
  });
}

function makeFakeAudioEl(): HTMLAudioElement {
  return {
    volume: 1,
    currentTime: 0,
    duration: 0,
    paused: true,
    src: "",
    play: vi.fn(() => Promise.resolve()),
    pause: vi.fn(),
  } as unknown as HTMLAudioElement;
}

function makeMemoryStorage(): Storage {
  const store = new Map<string, string>();
  return {
    getItem: (key: string) => store.get(key) ?? null,
    setItem: (key: string, value: string) => void store.set(key, value),
    removeItem: (key: string) => void store.delete(key),
    clear: () => store.clear(),
    key: (i: number) => Array.from(store.keys())[i] ?? null,
    get length() {
      return store.size;
    },
  } as Storage;
}

let player: PlayerStore;

beforeEach(() => {
  // attach() starts a real position-watchdog setInterval (see
  // player.svelte.ts) -- fake timers keep every PlayerStore instance
  // created in these tests (the shared `player` below, plus the ad-hoc ones
  // some persistence tests create) from leaking a live interval past the
  // test that created it. useRealTimers() in afterEach discards all of them
  // in one shot, so no per-test cleanup is needed at each call site.
  vi.useFakeTimers();
  // Node's built-in `localStorage` global throws without a configured
  // backing file, and it shadows happy-dom's -- swap in a plain in-memory
  // implementation so the "persists across instances" tests mean something.
  vi.stubGlobal("localStorage", makeMemoryStorage());
  // never hit the network in tests -- _loadCurrent's fetch should fail fast and silently
  vi.stubGlobal("fetch", vi.fn(() => Promise.reject(new Error("no network in tests"))));
  // Defaults to "nothing related" so existing end-of-queue tests keep
  // seeing the pre-autoplay-related behavior (playback just stops) unless a
  // test explicitly opts in with its own mockResolvedValue.
  vi.mocked(api.relatedTracks).mockReset().mockResolvedValue([]);
  player = new PlayerStore();
  player.attach(makeFakeAudioEl());
});

afterEach(() => {
  vi.useRealTimers();
});

describe("shuffle", () => {
  it("defaults to off", () => {
    expect(player.shuffle).toBe(false);
  });

  it("toggles on and off", () => {
    player.toggleShuffle();
    expect(player.shuffle).toBe(true);
    player.toggleShuffle();
    expect(player.shuffle).toBe(false);
  });

  it("persists across a new store instance", () => {
    player.toggleShuffle();
    expect(player.shuffle).toBe(true);
    const reloaded = new PlayerStore();
    expect(reloaded.shuffle).toBe(true);
  });

  it("does not touch the queue when there's nothing to shuffle", () => {
    const tracks = [makeTrack(1)];
    player.play(tracks[0], tracks);
    player.toggleShuffle();
    const before = [...player.queue];
    player.next(); // only one track total, nothing after it
    expect(player.queue).toEqual(before);
    expect(player.queueIndex).toBe(0); // unchanged, no loop
  });

  it("picks the next track from the remaining pool, never repeating or skipping", () => {
    const tracks = [1, 2, 3, 4, 5].map((id) => makeTrack(id));
    player.play(tracks[0], tracks);
    player.toggleShuffle();
    player.random = () => 0.999999; // force "last in pool" every time for a deterministic run

    const seen = new Set<number>([player.current!.id]);
    for (let i = 0; i < tracks.length - 1; i++) {
      player.next();
      expect(player.current).not.toBeNull();
      expect(seen.has(player.current!.id)).toBe(false); // never repeats a track
      seen.add(player.current!.id);
    }
    expect(seen.size).toBe(tracks.length); // every track was eventually played exactly once
  });

  it("shuffles the upcoming queue immediately on toggle, not lazily on next()", () => {
    // Deterministic random set *before* toggling, since the shuffle now
    // happens right at toggle time rather than waiting for the first next().
    const tracks = [1, 2, 3, 4, 5].map((id) => makeTrack(id));
    player.play(tracks[0], tracks);
    player.random = () => 0; // always swap-to-front -- a real permutation, not a no-op
    player.toggleShuffle();

    // "Up next" (player.upcoming) must differ from the original order right
    // away -- previously it silently stayed in original order until next()
    // was called at least once, which is exactly the reported bug.
    const upcomingIds = player.upcoming.map((t) => t.id);
    expect(upcomingIds).not.toEqual([2, 3, 4, 5]);
    expect(new Set(upcomingIds)).toEqual(new Set([2, 3, 4, 5])); // still the same tracks, just reordered
  });

  it("turning shuffle back off restores the original (pre-shuffle) order", () => {
    const tracks = [1, 2, 3, 4, 5].map((id) => makeTrack(id));
    player.play(tracks[0], tracks);
    player.random = () => 0; // guarantees a real, non-identity permutation
    player.toggleShuffle();
    expect(player.upcoming.map((t) => t.id)).not.toEqual([2, 3, 4, 5]); // sanity: it did shuffle

    player.toggleShuffle(); // off
    expect(player.upcoming.map((t) => t.id)).toEqual([2, 3, 4, 5]); // back to original order
    expect(player.current!.id).toBe(1); // still on the same track
  });

  it("restores original order relative to wherever playback currently is, not just the start", () => {
    const tracks = [1, 2, 3, 4, 5].map((id) => makeTrack(id));
    player.play(tracks[0], tracks);
    player.random = () => 0;
    player.toggleShuffle();
    player.next(); // now somewhere in the shuffled order, not necessarily track 2
    const playingId = player.current!.id;

    player.toggleShuffle(); // off
    expect(player.current!.id).toBe(playingId); // still playing the same track
    // and the full queue is back to the original 1..5 order
    expect(player.queue.map((t) => t.id)).toEqual([1, 2, 3, 4, 5]);
  });

  it("keeps previous() working after a shuffled next() (queue order is the source of truth)", () => {
    const tracks = [1, 2, 3].map((id) => makeTrack(id));
    player.play(tracks[0], tracks);
    player.toggleShuffle();
    player.random = () => 0.999999;

    player.next();
    const afterNext = player.current;
    player.previous();
    expect(player.current!.id).toBe(tracks[0].id); // back to the original first track

    player.next();
    expect(player.current!.id).toBe(afterNext!.id); // re-visiting gives the same (now-fixed) order
  });
});

describe("loop", () => {
  it("defaults to off and cycles off -> all -> one -> off", () => {
    expect(player.loop).toBe("off");
    player.cycleLoop();
    expect(player.loop).toBe("all");
    player.cycleLoop();
    expect(player.loop).toBe("one");
    player.cycleLoop();
    expect(player.loop).toBe("off");
  });

  it("persists across a new store instance", () => {
    player.cycleLoop();
    expect(player.loop).toBe("all");
    const reloaded = new PlayerStore();
    expect(reloaded.loop).toBe("all");
  });

  it("loop=off: next() at the end of the queue does nothing", () => {
    const tracks = [makeTrack(1), makeTrack(2)];
    player.play(tracks[0], tracks);
    player.next();
    expect(player.queueIndex).toBe(1);
    player.next(); // already at the last track
    expect(player.queueIndex).toBe(1);
    expect(player.current!.id).toBe(2);
  });

  it("loop=all: next() at the end of the queue wraps to the first track", () => {
    const tracks = [makeTrack(1), makeTrack(2), makeTrack(3)];
    player.play(tracks[0], tracks);
    player.cycleLoop(); // -> all
    player.next();
    player.next();
    expect(player.queueIndex).toBe(2);
    player.next(); // wraps
    expect(player.queueIndex).toBe(0);
    expect(player.current!.id).toBe(1);
  });

  it("loop=all: previous() at the start of the queue wraps to the last track", () => {
    const tracks = [makeTrack(1), makeTrack(2), makeTrack(3)];
    player.play(tracks[0], tracks);
    player.cycleLoop(); // -> all
    player.previous(); // already at index 0
    expect(player.queueIndex).toBe(2);
    expect(player.current!.id).toBe(3);
  });

  it("loop=one: track-ended replays the same track instead of advancing", () => {
    const tracks = [makeTrack(1), makeTrack(2)];
    player.play(tracks[0], tracks);
    player.cycleLoop();
    player.cycleLoop(); // -> one
    const audioEl = player.audioEl as unknown as { currentTime: number; play: () => Promise<void> };
    audioEl.currentTime = 42;

    player.onTrackEnded();

    expect(player.queueIndex).toBe(0); // did not advance
    expect(audioEl.currentTime).toBe(0); // seeked back to the start
    expect(audioEl.play).toHaveBeenCalled();
  });

  it("loop=off: track-ended advances to the next track", () => {
    const tracks = [makeTrack(1), makeTrack(2)];
    player.play(tracks[0], tracks);
    player.onTrackEnded();
    expect(player.queueIndex).toBe(1);
    expect(player.current!.id).toBe(2);
  });

  it("loop=all: track-ended past the last track wraps around", () => {
    const tracks = [makeTrack(1), makeTrack(2)];
    player.play(tracks[0], tracks);
    player.cycleLoop(); // -> all
    player.next(); // now at track 2 (last)
    player.onTrackEnded();
    expect(player.queueIndex).toBe(0);
    expect(player.current!.id).toBe(1);
  });
});

describe("unplayable (DRM) tracks are auto-skipped", () => {
  it("next() past a DRM track lands on the following playable track instead of freezing", () => {
    const tracks = [makeTrack(1), makeDrmTrack(2), makeTrack(3)];
    player.play(tracks[0], tracks);
    player.next();
    expect(player.queueIndex).toBe(2);
    expect(player.current!.id).toBe(3);
    expect(player.notice).toContain("Skipped");
  });

  it("onTrackEnded (autoplay) skips a DRM track the same way a manual next() would", () => {
    const tracks = [makeTrack(1), makeDrmTrack(2), makeTrack(3)];
    player.play(tracks[0], tracks);
    player.onTrackEnded();
    expect(player.queueIndex).toBe(2);
    expect(player.current!.id).toBe(3);
  });

  it("skips over multiple consecutive DRM tracks in one next()", () => {
    const tracks = [makeTrack(1), makeDrmTrack(2), makeDrmTrack(3), makeDrmTrack(4), makeTrack(5)];
    player.play(tracks[0], tracks);
    player.next();
    expect(player.queueIndex).toBe(4);
    expect(player.current!.id).toBe(5);
  });

  it("previous() skips a DRM track backwards, not forwards", () => {
    const tracks = [makeTrack(1), makeDrmTrack(2), makeTrack(3)];
    player.play(tracks[2], tracks); // start at the last track
    player.previous();
    expect(player.queueIndex).toBe(0);
    expect(player.current!.id).toBe(1);
  });

  it("playing a DRM track directly auto-skips forward within its context", () => {
    const tracks = [makeDrmTrack(1), makeTrack(2), makeTrack(3)];
    player.play(tracks[0], tracks);
    expect(player.queueIndex).toBe(1);
    expect(player.current!.id).toBe(2);
  });

  it("a queue that's entirely DRM stops gracefully instead of spinning forever", async () => {
    const tracks = [makeDrmTrack(1), makeDrmTrack(2), makeDrmTrack(3)];
    player.play(tracks[0], tracks);
    // play() itself tries to auto-skip forward from track 1; with everything
    // DRM-locked it should land at the last track, try (and, per the
    // beforeEach default, fail to find) related tracks to fall back to, and
    // only then give up there -- that fallback attempt is async, hence the
    // await below.
    expect(player.queueIndex).toBe(2);
    await vi.waitFor(() => expect(player.error).toBeTruthy());
  });

  it("does not push duplicate or intermediate (skipped-over) tracks into history", () => {
    const tracks = [makeTrack(1), makeDrmTrack(2), makeDrmTrack(3), makeTrack(4)];
    player.play(tracks[0], tracks);
    const historyBefore = player.history.length;
    player.next();
    expect(player.current!.id).toBe(4);
    // exactly one new entry for the landed-on track, not one per hop -- and
    // it's most-recent-first, so the new entry is at the front.
    expect(player.history.length).toBe(historyBefore + 1);
    expect(player.history[0].id).toBe(4);
  });
});

describe("shuffle and loop combined", () => {
  it("loop=all with shuffle keeps producing every track once per lap without repeats", () => {
    const tracks = [1, 2, 3, 4].map((id) => makeTrack(id));
    player.play(tracks[0], tracks);
    player.toggleShuffle();
    player.cycleLoop(); // -> all
    player.random = () => 0.5;

    const seenFirstLap = new Set<number>([player.current!.id]);
    for (let i = 0; i < tracks.length - 1; i++) {
      player.next();
      seenFirstLap.add(player.current!.id);
    }
    expect(seenFirstLap.size).toBe(tracks.length);

    // one more next() should wrap back to the start of the (now-fixed) shuffled order
    player.next();
    expect(player.queueIndex).toBe(0);
  });
});

describe("playback persistence across app restarts", () => {
  it("restores queue, index, and position on a new store instance, without autoplaying", async () => {
    const tracks = [makeTrack(1), makeTrack(2), makeTrack(3)];
    player.play(tracks[0], tracks);
    player.next(); // now on track 2
    const audioEl = player.audioEl as unknown as { currentTime: number };
    audioEl.currentTime = 42;
    player.savePositionTick(true); // force an immediate save at position 42s

    const restored = new PlayerStore();
    const fakeEl = makeFakeAudioEl();
    restored.attach(fakeEl);
    await restored.restore();

    expect(restored.queueIndex).toBe(1);
    expect(restored.current!.id).toBe(2);
    expect(fakeEl.play).not.toHaveBeenCalled(); // restoring should never autoplay
  });

  it("restore() itself never touches the network -- loading is deferred to the first play", async () => {
    const tracks = [makeTrack(1), makeTrack(2)];
    player.play(tracks[0], tracks);
    (player.audioEl as unknown as { currentTime: number }).currentTime = 30;
    player.savePositionTick(true);

    const fetchMock = globalThis.fetch as unknown as ReturnType<typeof vi.fn>;
    fetchMock.mockClear();

    const restored = new PlayerStore();
    const fakeEl = makeFakeAudioEl();
    restored.attach(fakeEl);
    await restored.restore();

    expect(fetchMock).not.toHaveBeenCalled(); // no request just from restoring
    expect(restored.pendingSeek).not.toBeNull();

    restored.toggle(); // first interaction actually loads it
    expect(fetchMock).toHaveBeenCalledTimes(1);
    expect(restored.pendingSeek).toBeNull(); // consumed
  });

  it("moving off the restored track (next/previous/play) clears the pending seek", async () => {
    const tracks = [makeTrack(1), makeTrack(2), makeTrack(3)];
    player.play(tracks[0], tracks);
    (player.audioEl as unknown as { currentTime: number }).currentTime = 15;
    player.savePositionTick(true);

    const restored = new PlayerStore();
    const fakeEl = makeFakeAudioEl();
    restored.attach(fakeEl);
    await restored.restore();
    expect(restored.pendingSeek).not.toBeNull();

    restored.next();
    expect(restored.pendingSeek).toBeNull();
  });

  it("does nothing when there's no saved playback state", async () => {
    const fresh = new PlayerStore();
    const fakeEl = makeFakeAudioEl();
    fresh.attach(fakeEl);
    await fresh.restore();
    expect(fresh.queue).toEqual([]);
    expect(fresh.queueIndex).toBe(-1);
  });

  it("clearPersistedPlayback() prevents a later restore from picking anything up", async () => {
    const tracks = [makeTrack(1), makeTrack(2)];
    player.play(tracks[0], tracks);
    player.savePositionTick(true);
    player.clearPersistedPlayback();

    const restored = new PlayerStore();
    const fakeEl = makeFakeAudioEl();
    restored.attach(fakeEl);
    await restored.restore();
    expect(restored.queue).toEqual([]);
  });
});

describe("live position tracking (currentTime/duration/seek)", () => {
  it("duration falls back to the current track's own metadata before the element reports one", () => {
    player.play(makeTrack(1, { duration: 180000 })); // 180s
    expect(player.duration).toBe(180);
  });

  it("onDurationChange() overrides the metadata fallback once the element has a real duration", () => {
    player.play(makeTrack(1, { duration: 180000 }));
    (player.audioEl as unknown as { duration: number }).duration = 172.4;
    player.onDurationChange();
    expect(player.duration).toBeCloseTo(172.4);
  });

  it("onTimeUpdate() syncs currentTime from the element", () => {
    player.play(makeTrack(1));
    (player.audioEl as unknown as { currentTime: number }).currentTime = 42;
    player.onTimeUpdate();
    expect(player.currentTime).toBe(42);
  });

  it("pendingSeek overrides the live position until it's consumed", async () => {
    const tracks = [makeTrack(1), makeTrack(2)];
    const saved = { queue: tracks, originalQueue: tracks, queueIndex: 0, positionSeconds: 91 };
    localStorage.setItem("sc-desktop:playback", JSON.stringify(saved));

    const restored = new PlayerStore();
    restored.attach(makeFakeAudioEl());
    await restored.restore();

    expect(restored.pendingSeek).toBe(91);
    // pendingSeek wins even though the (never-loaded) element itself is still at 0
    expect(restored.currentTime).toBe(91);
  });

  it("seek() updates currentTime immediately, without waiting for a timeupdate event", () => {
    player.play(makeTrack(1));
    player.seek(55);
    expect(player.currentTime).toBe(55);
    expect((player.audioEl as unknown as { currentTime: number }).currentTime).toBe(55);
  });

  it("seek() does nothing without a known duration", () => {
    player.play(makeTrack(1, { duration: 0 }));
    player.seek(55);
    expect(player.currentTime).toBe(0);
  });

  // The bug this whole suite exists to pin down: volume and playback
  // position are two unrelated things, and nothing about changing one
  // should ever be able to touch the other.
  it("setVolume() never touches currentTime, duration, or the element's currentTime", () => {
    player.play(makeTrack(1, { duration: 120000 }));
    player.seek(30);
    const audioEl = player.audioEl as unknown as { currentTime: number; volume: number };
    expect(player.currentTime).toBe(30);

    // Simulate a fast slider drag: many rapid setVolume calls in a row.
    for (let i = 0; i <= 20; i++) player.setVolume(i / 20);

    expect(player.currentTime).toBe(30);
    expect(audioEl.currentTime).toBe(30);
    expect(player.duration).toBe(120);
    expect(audioEl.volume).toBe(1);
  });

  it("toggleMute() never touches currentTime or duration either", () => {
    player.play(makeTrack(1));
    player.seek(17);
    player.toggleMute();
    player.toggleMute();
    expect(player.currentTime).toBe(17);
  });
});

describe("position watchdog (self-heals a stalled timeupdate without disturbing seeks)", () => {
  it("does nothing while timeupdate keeps arriving normally", () => {
    player.play(makeTrack(1));
    player.isPlaying = true;
    const audioEl = player.audioEl as unknown as { currentTime: number };

    audioEl.currentTime = 5;
    player.onTimeUpdate();
    vi.advanceTimersByTime(500);
    audioEl.currentTime = 10;
    player.onTimeUpdate();
    vi.advanceTimersByTime(500);

    expect(player.currentTime).toBe(10); // exactly what the last real tick reported
  });

  it("resyncs currentTime from the element if timeupdate goes silent for ~1s while playing", () => {
    player.play(makeTrack(1));
    player.isPlaying = true;
    const audioEl = player.audioEl as unknown as { currentTime: number };

    audioEl.currentTime = 5;
    player.onTimeUpdate();

    // Simulate playback continuing (the real element's position keeps
    // advancing) while the timeupdate EVENT itself goes silent -- confirmed
    // live that a flood of rapid, unrelated UI events (e.g. dragging the
    // volume slider) can starve the event loop long enough for this to
    // happen, even though the audio itself never stopped.
    audioEl.currentTime = 23; // no onTimeUpdate() call to go with this

    vi.advanceTimersByTime(1000);

    expect(player.currentTime).toBe(23); // watchdog caught it without needing a track change
  });

  it("does not resync while paused", () => {
    player.play(makeTrack(1));
    player.isPlaying = false;
    const audioEl = player.audioEl as unknown as { currentTime: number };
    audioEl.currentTime = 5;
    player.onTimeUpdate();

    audioEl.currentTime = 99;
    vi.advanceTimersByTime(3000);

    expect(player.currentTime).toBe(5); // watchdog only acts while isPlaying
  });

  it("does not clobber a fresh seek, even once the watchdog fires again shortly after", () => {
    player.play(makeTrack(1));
    player.isPlaying = true;
    const audioEl = player.audioEl as unknown as { currentTime: number };
    audioEl.currentTime = 5;
    player.onTimeUpdate();

    vi.advanceTimersByTime(1500); // watchdog has now had at least one chance to fire on the pre-seek position

    player.seek(77);
    expect(player.currentTime).toBe(77);

    vi.advanceTimersByTime(500); // less than the watchdog's ~900ms staleness threshold
    expect(player.currentTime).toBe(77); // still exactly what seek() set

    vi.advanceTimersByTime(1000); // now past the threshold -- watchdog ticks again
    // Nothing changed audioEl.currentTime since the seek, so re-reading it is a no-op.
    expect(player.currentTime).toBe(77);
    expect(audioEl.currentTime).toBe(77);
  });

  it("picks up real playback advancement after a seek, once genuinely stalled", () => {
    player.play(makeTrack(1));
    player.isPlaying = true;
    player.seek(10);

    const audioEl = player.audioEl as unknown as { currentTime: number };
    audioEl.currentTime = 15; // playback continued past the seek point, event stalled again

    vi.advanceTimersByTime(1000);

    expect(player.currentTime).toBe(15);
  });

  it("destroy() stops the watchdog so it can't fire against a torn-down element", () => {
    player.play(makeTrack(1));
    player.isPlaying = true;
    const audioEl = player.audioEl as unknown as { currentTime: number };
    audioEl.currentTime = 5;
    player.onTimeUpdate();

    player.destroy();
    audioEl.currentTime = 50;
    vi.advanceTimersByTime(5000);

    expect(player.currentTime).toBe(5); // no watchdog left to pick up the change
  });
});

describe("autoplay related tracks when the queue ends", () => {
  it("next() past the end of the queue plays related tracks instead of stopping", async () => {
    const tracks = [makeTrack(1)];
    player.play(tracks[0], tracks);
    vi.mocked(api.relatedTracks).mockResolvedValue([makeTrack(99)]);

    player.next();
    await vi.waitFor(() => expect(player.queue.length).toBe(2));

    expect(player.queueIndex).toBe(1);
    expect(player.current!.id).toBe(99);
    expect(player.queue.map((t) => t.id)).toEqual([1, 99]);
    expect(player.notice).toContain("related");
  });

  it("onTrackEnded() at the last track also triggers the fallback (the real 'song finished' path)", async () => {
    const tracks = [makeTrack(1)];
    player.play(tracks[0], tracks);
    vi.mocked(api.relatedTracks).mockResolvedValue([makeTrack(99)]);

    player.onTrackEnded();
    await vi.waitFor(() => expect(player.current!.id).toBe(99));
  });

  it("does nothing when there are no related tracks -- playback just stops, as before", async () => {
    const tracks = [makeTrack(1)];
    player.play(tracks[0], tracks);
    vi.mocked(api.relatedTracks).mockResolvedValue([]); // matches the beforeEach default, explicit here for clarity

    player.next();
    await vi.waitFor(() => expect(api.relatedTracks).toHaveBeenCalled());
    await Promise.resolve();

    expect(player.queue.length).toBe(1);
    expect(player.queueIndex).toBe(0);
  });

  it("filters out related tracks already present in the queue", async () => {
    const tracks = [makeTrack(1), makeTrack(2)];
    player.play(tracks[0], tracks);
    vi.mocked(api.relatedTracks).mockResolvedValue([makeTrack(2), makeTrack(3)]); // track 2 is already in the queue

    player.next(); // advances 1 -> 2 normally, nothing related fetched yet
    expect(player.current!.id).toBe(2);

    player.next(); // now at the real end -- fetches related, should only add track 3
    await vi.waitFor(() => expect(player.queue.length).toBe(3));

    expect(player.current!.id).toBe(3);
    expect(player.queue.map((t) => t.id)).toEqual([1, 2, 3]);
  });

  it("loop=all wraps around instead of ever fetching related tracks", () => {
    const tracks = [makeTrack(1), makeTrack(2)];
    player.play(tracks[0], tracks);
    player.cycleLoop(); // -> all
    player.next(); // -> track 2

    player.next(); // wraps back to track 1

    expect(player.queueIndex).toBe(0);
    expect(api.relatedTracks).not.toHaveBeenCalled();
  });

  it("discards a stale related-tracks fetch if a different track was explicitly played in the meantime", async () => {
    const tracks = [makeTrack(1)];
    player.play(tracks[0], tracks);
    let resolveRelated: (v: Track[]) => void = () => {};
    vi.mocked(api.relatedTracks).mockReturnValue(new Promise((r) => (resolveRelated = r)));

    player.next(); // kicks off the related-tracks fetch, left pending

    const other = makeTrack(2);
    player.play(other); // a brand-new, unrelated load starts while that fetch is still in flight

    resolveRelated([makeTrack(99)]);
    await Promise.resolve();
    await Promise.resolve();

    expect(player.current!.id).toBe(2); // untouched by the now-stale related-tracks result
    expect(player.queue).toEqual([other]);
  });
});
