export type User = {
  id: number;
  username: string | null;
  avatar_url: string | null;
  permalink: string | null;
  permalink_url: string | null;
};

/** The real @handle -- `username` is a display name, not the URL slug. */
export function handleOf(u: { permalink: string | null; permalink_url: string | null; username: string | null }): string | null {
  if (u.permalink) return u.permalink;
  if (u.permalink_url) {
    const last = u.permalink_url.split("/").filter(Boolean).pop();
    if (last) return last;
  }
  return u.username;
}

export type Track = {
  id: number;
  title: string | null;
  permalink_url: string | null;
  artwork_url: string | null;
  duration: number | null;
  genre: string | null;
  user: User | null;
  streamable: boolean;
  likes_count: number | null;
  reposts_count: number | null;
  comment_count: number | null;
  playback_count: number | null;
  created_at: string | null;
  media: { transcodings: { format: { protocol: string | null; mime_type: string | null } | null }[] };
  waveform_url: string | null;
};

/**
 * Mirrors the backend's Track::candidate_transcodings selection (see
 * src-tauri/src/soundcloud/models.rs): only progressive or plain
 * (unencrypted) MP3-HLS are playable. Everything else -- real DRM
 * (Apple FairPlay / Widevine encrypted-hls) or AAC-only HLS -- isn't.
 * Used to show a lock icon up front instead of letting the user hit play
 * and get a failure.
 */
export function isPlayable(track: Track): boolean {
  const transcodings = track.media?.transcodings ?? [];
  // Any encrypted-HLS listing at all means real DRM -- confirmed live that
  // plain progressive/hls-mp3 listed alongside it are vestigial and 404.
  const hasDrm = transcodings.some((t) => t.format?.protocol?.includes("encrypted"));
  if (hasDrm) return false;
  return transcodings.some((t) => {
    const protocol = t.format?.protocol;
    const mime = t.format?.mime_type;
    return protocol === "progressive" || (protocol === "hls" && mime === "audio/mpeg");
  });
}

export type Comment = {
  id: number;
  body: string | null;
  created_at: string | null;
  user: User | null;
};

export type Profile = {
  id: number;
  username: string | null;
  full_name: string | null;
  avatar_url: string | null;
  permalink: string | null;
  permalink_url: string | null;
  description: string | null;
  city: string | null;
  country_code: string | null;
  followers_count: number | null;
  followings_count: number | null;
  track_count: number | null;
  likes_count: number | null;
  visuals: { visuals: { visual_url: string | null }[] } | null;
};

export function bannerUrl(p: Profile): string | null {
  return p.visuals?.visuals[0]?.visual_url ?? null;
}

export type UserComment = {
  id: number;
  body: string | null;
  created_at: string | null;
  track_id: number | null;
  track_title: string | null;
};

export function timeAgo(iso: string | null): string {
  if (!iso) return "";
  const diffMs = Date.now() - new Date(iso).getTime();
  const days = Math.floor(diffMs / (1000 * 60 * 60 * 24));
  if (days < 1) return "today";
  if (days < 30) return `${days}d ago`;
  const months = Math.floor(days / 30);
  if (months < 12) return `${months}mo ago`;
  return `${Math.floor(months / 12)}y ago`;
}

export type Playlist = {
  id: number;
  title: string | null;
  permalink_url: string | null;
  artwork_url: string | null;
  track_count: number | null;
  tracks: Track[];
};

/**
 * SoundCloud-generated set ("Your Mix N", "Related tracks: ...", weekly
 * mood mixes) as returned inside a mixed-selections module. Not a real
 * playlist -- `id` is a string urn, there's no `/playlists/{id}` resource
 * for it, and `tracks` only holds id stubs that need a separate hydration
 * call (see `api.systemPlaylistTracks`).
 */
export type SystemPlaylist = {
  id: string;
  title: string | null;
  artwork_url: string | null;
  calculated_artwork_url: string | null;
  permalink_url: string | null;
  tracks: { id: number }[];
};

export function isSystemPlaylist(p: Playlist | SystemPlaylist): p is SystemPlaylist {
  return typeof p.id === "string";
}

export function selectionArtwork(p: Playlist | SystemPlaylist): string | null {
  if (isSystemPlaylist(p)) return p.artwork_url ?? p.calculated_artwork_url;
  return p.artwork_url;
}

/** SoundCloud's homepage curation modules (Trending by genre, etc). */
export type Selection = {
  urn: string | null;
  title: string | null;
  items: { collection: (Playlist | SystemPlaylist)[] };
};

/**
 * One item from the unified "All" search (see `api.searchAll`) -- tracks,
 * artists, and playlists interleaved in relevance order, each tagged with
 * `kind` by the Rust side (see `SearchResultItem` in
 * src-tauri/src/soundcloud/models.rs) so the frontend can discriminate
 * them, since unlike Playlist/SystemPlaylist they don't have a naturally
 * distinguishing field (all three have plain numeric ids).
 */
export type SearchResultItem = ({ kind: "track" } & Track) | ({ kind: "user" } & Profile) | ({ kind: "playlist" } & Playlist);

export function formatDuration(ms: number | null): string {
  if (!ms || ms <= 0) return "--:--";
  const totalSeconds = Math.floor(ms / 1000);
  const minutes = Math.floor(totalSeconds / 60);
  const seconds = totalSeconds % 60;
  return `${minutes}:${seconds.toString().padStart(2, "0")}`;
}

/**
 * The mini player window has no audio element and no seeded likes/following
 * stores of its own -- it's a thin remote control. The main window pushes
 * this snapshot over a Tauri event (see PlayerBar.svelte's emitter effect)
 * whenever anything relevant changes, and again on demand when the mini
 * player first mounts (see "miniplayer:request-state"). `loop` duplicates
 * player.svelte.ts's LoopMode union rather than importing it, to avoid a
 * circular import (player.svelte.ts already imports Track from this file).
 */
export type MiniPlayerState = {
  track: Track | null;
  isPlaying: boolean;
  position: number;
  duration: number;
  shuffle: boolean;
  loop: "off" | "all" | "one";
  isLiked: boolean;
  isFollowing: boolean;
  volume: number;
  /** SoundCloud's real per-track amplitude envelope (see Track.waveform_url), resampled to a fixed bar count -- static per track, not a live analyser. */
  waveform: number[];
  upcoming: Track[];
};

/**
 * Sent from the mini player to the main window. Playback transport goes
 * through here because the real <audio> element only exists in the main
 * window; like/follow use "toggle" verbs (not separate like/unlike) so the
 * two windows never need to agree on whose idea of the current state is
 * newer.
 */
export type MiniPlayerCommand =
  | { action: "toggle" }
  | { action: "next" }
  | { action: "previous" }
  | { action: "shuffle" }
  | { action: "cycleLoop" }
  | { action: "toggleLike" }
  | { action: "toggleFollow" }
  | { action: "seek"; position: number }
  | { action: "toggleMute" }
  | { action: "setVolume"; value: number };

/** Sent from the mini player when the user wants back into the full app -- optionally with a track to open directly. */
export type MiniPlayerShowMain = { trackId: number | null };
