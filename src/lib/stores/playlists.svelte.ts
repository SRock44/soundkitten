import { api } from "../api";
import type { Playlist, Track } from "../types";

/** Canonical list of the logged-in user's own + liked playlists (mirrors
 * `likes`/`following`'s "one global store, imported directly wherever
 * needed" pattern, rather than prop-drilling mutation callbacks through
 * every component that can trigger a playlist change -- Home, Search,
 * Profile, and the track context menu all need to create/add-to/rename/
 * delete playlists, several layers deep in different trees). */
class PlaylistsStore {
  items = $state<Playlist[]>([]);

  seed(playlists: Playlist[]) {
    this.items = playlists;
  }

  async create(title: string, trackIds: number[] = []): Promise<Playlist> {
    const created = await api.createPlaylistV2(title, trackIds);
    this.items = [created, ...this.items];
    return created;
  }

  async rename(playlistId: number, title: string): Promise<Playlist> {
    const current = await api.playlist(playlistId);
    const updated = await api.updatePlaylistV2(playlistId, title, current.tracks.map((t) => t.id));
    this._replace(updated);
    return updated;
  }

  /**
   * SoundCloud's classic API replaces a playlist's whole track list on
   * update -- there's no per-track add endpoint. Refetches the real current
   * track list first rather than trusting `items`' possibly-stale/metadata-
   * only copy (the listing endpoint doesn't return hydrated tracks), so a
   * track added from another device or soundcloud.com itself since the
   * last local fetch doesn't get silently wiped out.
   */
  async addTrack(playlistId: number, track: Track): Promise<Playlist> {
    const current = await api.playlist(playlistId);
    if (current.tracks.some((t) => t.id === track.id)) return current;
    const updated = await api.updatePlaylistV2(playlistId, current.title ?? "Untitled", [...current.tracks.map((t) => t.id), track.id]);
    this._replace(updated);
    return updated;
  }

  /** Mirrors addTrack()'s stale-data caution. */
  async removeTrack(playlistId: number, trackId: number): Promise<Playlist> {
    const current = await api.playlist(playlistId);
    const trackIds = current.tracks.filter((t) => t.id !== trackId).map((t) => t.id);
    const updated = await api.updatePlaylistV2(playlistId, current.title ?? "Untitled", trackIds);
    this._replace(updated);
    return updated;
  }

  async remove(playlistId: number): Promise<void> {
    await api.deletePlaylistV2(playlistId);
    this.items = this.items.filter((p) => p.id !== playlistId);
  }

  private _replace(updated: Playlist) {
    this.items = this.items.map((p) => (p.id === updated.id ? updated : p));
  }
}

export const playlistsStore = new PlaylistsStore();
