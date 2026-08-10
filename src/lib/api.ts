import { invoke } from "@tauri-apps/api/core";
import type { Comment, FeedEntry, Playlist, Profile, SearchResultItem, Selection, Track, UserComment } from "./types";

export const api = {
  openMiniPlayer: () => invoke<void>("open_mini_player"),
  /** Temporary: pipes a log line straight to the `tauri dev` terminal, for tracking down the right-click context-menu issue. */
  debugLog: (msg: string) => invoke<void>("debug_log", { msg }),
  isLoggedIn: () => invoke<boolean>("is_logged_in"),
  startLogin: () => invoke<void>("start_login"),
  verifyAuth: () => invoke<boolean>("verify_auth"),
  logout: () => invoke<void>("logout"),
  search: (query: string) => invoke<Track[]>("sc_search", { query }),
  likes: () => invoke<Track[]>("sc_likes"),
  playlists: () => invoke<Playlist[]>("sc_playlists"),
  me: () => invoke<Profile>("sc_me"),
  userProfile: (userId: number) => invoke<Profile>("sc_user_profile", { userId }),
  userTracks: (userId: number) => invoke<Track[]>("sc_user_tracks", { userId }),
  searchUsers: (query: string) => invoke<Profile[]>("sc_search_users", { query }),
  searchAll: (query: string) => invoke<SearchResultItem[]>("sc_search_all", { query }),
  feed: () => invoke<FeedEntry[]>("sc_feed"),
  likeTrack: (trackId: number) => invoke<void>("sc_like_track", { trackId }),
  unlikeTrack: (trackId: number) => invoke<void>("sc_unlike_track", { trackId }),
  repostTrack: (trackId: number) => invoke<void>("sc_repost_track", { trackId }),
  unrepostTrack: (trackId: number) => invoke<void>("sc_unrepost_track", { trackId }),
  trackComments: (trackId: number) => invoke<Comment[]>("sc_track_comments", { trackId }),
  postComment: (trackId: number, body: string) => invoke<Comment>("sc_post_comment", { trackId, body }),
  userReposts: (userId: number) => invoke<Track[]>("sc_user_reposts", { userId }),
  userPlaylists: (userId: number) => invoke<Playlist[]>("sc_user_playlists", { userId }),
  userFollowers: (userId: number) => invoke<Profile[]>("sc_user_followers", { userId }),
  userFollowings: (userId: number) => invoke<Profile[]>("sc_user_followings", { userId }),
  myFollowingsIds: () => invoke<number[]>("sc_my_followings_ids"),
  userComments: (userId: number) => invoke<UserComment[]>("sc_user_comments", { userId }),
  playlist: (playlistId: number) => invoke<Playlist>("sc_playlist", { playlistId }),
  mixedSelections: () => invoke<Selection[]>("sc_mixed_selections"),
  systemPlaylistTracks: (trackIds: number[]) => invoke<Track[]>("sc_system_playlist_tracks", { trackIds }),
  setClientIdOverride: (id: string) => invoke<void>("set_client_id_override", { id }),
  clearClientIdOverride: () => invoke<void>("clear_client_id_override"),
  getClientIdOverride: () => invoke<string | null>("get_client_id_override"),

  // Official OAuth (second, optional login), used only for real in-app
  // like/follow writes, which are DataDome-blocked on the unofficial API.
  isOfficialConnected: () => invoke<boolean>("is_official_connected"),
  startOfficialLogin: () => invoke<void>("start_official_login"),
  disconnectOfficialLogin: () => invoke<void>("disconnect_official_login"),
  likeTrackV2: (trackId: number) => invoke<void>("sc_like_track_v2", { trackId }),
  unlikeTrackV2: (trackId: number) => invoke<void>("sc_unlike_track_v2", { trackId }),
  likePlaylistV2: (playlistId: number) => invoke<void>("sc_like_playlist_v2", { playlistId }),
  unlikePlaylistV2: (playlistId: number) => invoke<void>("sc_unlike_playlist_v2", { playlistId }),
  followUserV2: (userId: number) => invoke<void>("sc_follow_user_v2", { userId }),
  unfollowUserV2: (userId: number) => invoke<void>("sc_unfollow_user_v2", { userId }),
  createPlaylistV2: (title: string, trackIds: number[]) => invoke<Playlist>("sc_create_playlist_v2", { title, trackIds }),
  updatePlaylistV2: (playlistId: number, title: string, trackIds: number[]) =>
    invoke<Playlist>("sc_update_playlist_v2", { playlistId, title, trackIds }),
  deletePlaylistV2: (playlistId: number) => invoke<void>("sc_delete_playlist_v2", { playlistId }),
};

// Tauri's custom URI scheme handlers are addressed differently per
// platform: WebView2 (Windows) requires the scheme remapped to an
// http://<scheme>.localhost virtual host, while WKWebView (macOS/iOS) and
// WebKitGTK (Linux) address the scheme directly. The Rust handler
// (playback.rs) only reads the request path, so it's identical either way
// -- only this URL construction needs to branch. Confirmed live: the
// Windows-only form silently fails to route on macOS ("Load failed" on
// every play attempt), since WKWebView just treats "sc-stream.localhost"
// as a literal, unresolvable hostname instead of the custom protocol.
export function streamUrl(trackId: number): string {
  const isWindows = navigator.userAgent.includes("Windows");
  return isWindows ? `http://sc-stream.localhost/track/${trackId}` : `sc-stream://localhost/track/${trackId}`;
}
