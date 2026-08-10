import { api } from "../api";
import { officialAuth } from "./officialAuth.svelte";

/** Tracks which track ids the current user has liked, so like state is
 * consistent across the whole app without refetching per track. */
class LikesStore {
  ids = $state(new Set<number>());
  private busyIds = $state(new Set<number>());

  seed(likedTrackIds: number[]) {
    this.ids = new Set(likedTrackIds);
  }

  has(trackId: number): boolean {
    return this.ids.has(trackId);
  }

  set(trackId: number, liked: boolean) {
    const next = new Set(this.ids);
    if (liked) next.add(trackId);
    else next.delete(trackId);
    this.ids = next;
  }

  isBusy(trackId: number): boolean {
    return this.busyIds.has(trackId);
  }

  /**
   * Shared by PlayerBar, TrackDetail, and TrackRow's context menu -- the
   * ensureConnected -> write -> update-store flow used to be duplicated in
   * each of them (unofficial-API like writes are DataDome-blocked, see
   * docs/oauth-migration.md, official OAuth's /likes/tracks/{id} is the
   * only path that works). Resolves "declined" if the user chose not to
   * connect official OAuth (callers decide the fallback, e.g. opening the
   * track on soundcloud.com); throws on a real write failure so callers can
   * surface their own error text. No-ops (resolves "ok") if a toggle for
   * this track is already in flight, matching the old per-component
   * busy-guards this replaces.
   */
  async toggle(track: { id: number }): Promise<"ok" | "declined"> {
    if (this.isBusy(track.id)) return "ok";
    this.busyIds = new Set(this.busyIds).add(track.id);
    try {
      const connected = await officialAuth.ensureConnected();
      if (!connected) return "declined";
      const next = !this.has(track.id);
      if (next) await api.likeTrackV2(track.id);
      else await api.unlikeTrackV2(track.id);
      this.set(track.id, next);
      return "ok";
    } finally {
      const b = new Set(this.busyIds);
      b.delete(track.id);
      this.busyIds = b;
    }
  }
}

export const likes = new LikesStore();
