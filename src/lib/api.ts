import { invoke } from "@tauri-apps/api/core";
import type { Comment, Playlist, Profile, Selection, Track, UserComment } from "./types";

export const api = {
  isLoggedIn: () => invoke<boolean>("is_logged_in"),
  startLogin: () => invoke<void>("start_login"),
  verifyAuth: () => invoke<boolean>("verify_auth"),
  logout: () => invoke<void>("logout"),
  setManualToken: (token: string) => invoke<void>("set_manual_token", { token }),
  search: (query: string) => invoke<Track[]>("sc_search", { query }),
  likes: () => invoke<Track[]>("sc_likes"),
  playlists: () => invoke<Playlist[]>("sc_playlists"),
  me: () => invoke<Profile>("sc_me"),
  userProfile: (userId: number) => invoke<Profile>("sc_user_profile", { userId }),
  userTracks: (userId: number) => invoke<Track[]>("sc_user_tracks", { userId }),
  searchUsers: (query: string) => invoke<Profile[]>("sc_search_users", { query }),
  feed: () => invoke<Track[]>("sc_feed"),
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
};

export function streamUrl(trackId: number): string {
  return `http://sc-stream.localhost/track/${trackId}`;
}
