export type PlaylistView = "tiles" | "rows";

const PLAYLIST_VIEW_KEY = "sc-desktop:playlist-view";

// Same defensive guard as settings.svelte.ts / player.svelte.ts -- keeps
// this a nice-to-have rather than something that can throw during startup.
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

function loadPlaylistView(): PlaylistView {
  return safeStorageGet(PLAYLIST_VIEW_KEY) === "rows" ? "rows" : "tiles";
}

/** Global (not per-section) tile/row preference for playlist and mix
 * listings -- shared by Home.svelte's Playlists/Selections sections and
 * Profile.svelte's Playlists tab, so switching it in one place is
 * consistent everywhere rather than a per-shelf setting. */
class ViewModeStore {
  playlistView = $state(loadPlaylistView());

  setPlaylistView(v: PlaylistView) {
    this.playlistView = v;
    safeStorageSet(PLAYLIST_VIEW_KEY, v);
  }
}

export const viewMode = new ViewModeStore();
