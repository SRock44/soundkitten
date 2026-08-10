import { api } from "../api";
import { officialAuth } from "./officialAuth.svelte";

/** Tracks which user ids the current user follows, mirroring the `likes`
 * store's pattern. Seeded from the real (read-only) SoundCloud API, and
 * updated locally by `set()` when a follow/unfollow write succeeds via
 * official OAuth; unofficial-API follow writes are blocked by SoundCloud's
 * DataDome bot-protection (see src-tauri/src/soundcloud/api.rs) and never
 * reach this store. */
class FollowingStore {
  ids = $state(new Set<number>());
  private busyIds = $state(new Set<number>());

  seed(followedUserIds: number[]) {
    this.ids = new Set(followedUserIds);
  }

  has(userId: number): boolean {
    return this.ids.has(userId);
  }

  set(userId: number, following: boolean) {
    const next = new Set(this.ids);
    if (following) next.add(userId);
    else next.delete(userId);
    this.ids = next;
  }

  isBusy(userId: number): boolean {
    return this.busyIds.has(userId);
  }

  /** Mirrors LikesStore.toggle() -- see its doc comment for the full
   * rationale. Shared by PlayerBar, FollowButton, and TrackRow's context
   * menu. */
  async toggle(user: { id: number }): Promise<"ok" | "declined"> {
    if (this.isBusy(user.id)) return "ok";
    this.busyIds = new Set(this.busyIds).add(user.id);
    try {
      const connected = await officialAuth.ensureConnected();
      if (!connected) return "declined";
      const next = !this.has(user.id);
      if (next) await api.followUserV2(user.id);
      else await api.unfollowUserV2(user.id);
      this.set(user.id, next);
      return "ok";
    } finally {
      const b = new Set(this.busyIds);
      b.delete(user.id);
      this.busyIds = b;
    }
  }
}

export const following = new FollowingStore();
