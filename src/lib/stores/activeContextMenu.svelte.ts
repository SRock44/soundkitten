/** Ensures only one TrackRow context menu is open across the whole app at
 * a time. Each TrackRow keeps its own local `menuPos` (see TrackRow.svelte)
 * -- there's no single shared "which menu is open" state otherwise, so
 * right-clicking row B while row A's menu was still open left both
 * mounted simultaneously. A shared, monotonically increasing token fixes
 * this purely reactively (no event-timing games): opening a menu bumps
 * the token and remembers it locally, and a row's menu is only actually
 * shown while its remembered token still matches the current one -- the
 * moment a different row opens, every other row's comparison goes false
 * and its menu disappears on its own. */
class ActiveContextMenuStore {
  token = $state(0);

  /** Call when opening a menu; returns the token this specific menu should remember as "mine". */
  open(): number {
    this.token += 1;
    return this.token;
  }
}

export const activeContextMenu = new ActiveContextMenuStore();
