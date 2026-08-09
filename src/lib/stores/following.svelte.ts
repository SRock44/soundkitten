/** Tracks which user ids the current user follows, mirroring the `likes`
 * store's pattern. Seeded from the real (read-only) SoundCloud API, and
 * updated locally by `set()` when a follow/unfollow write succeeds via
 * official OAuth (see FollowButton.svelte); unofficial-API follow writes
 * are blocked by SoundCloud's DataDome bot-protection (see
 * src-tauri/src/soundcloud/api.rs) and never reach this store. */
class FollowingStore {
  ids = $state(new Set<number>());

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
}

export const following = new FollowingStore();
