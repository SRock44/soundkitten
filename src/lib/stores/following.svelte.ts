/** Tracks which user ids the current user follows, mirroring the `likes`
 * store's pattern. Read-only against the real SoundCloud API -- follow
 * writes are blocked by SoundCloud's DataDome bot-protection (see
 * src-tauri/src/soundcloud/api.rs), so this only ever reflects what was
 * true at last seed, never a locally-faked toggle. */
class FollowingStore {
  ids = $state(new Set<number>());

  seed(followedUserIds: number[]) {
    this.ids = new Set(followedUserIds);
  }

  has(userId: number): boolean {
    return this.ids.has(userId);
  }
}

export const following = new FollowingStore();
