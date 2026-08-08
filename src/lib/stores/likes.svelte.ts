/** Tracks which track ids the current user has liked, so like state is
 * consistent across the whole app without refetching per track. */
class LikesStore {
  ids = $state(new Set<number>());

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
}

export const likes = new LikesStore();
