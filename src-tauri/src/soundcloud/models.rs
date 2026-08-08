//! Typed, tolerant models for the unofficial SoundCloud v2 API.
//! All fields beyond the ones we actually use are `Option` or ignored,
//! so schema drift (SoundCloud adding/removing fields) doesn't break parsing.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    /// Display name (NOT the @handle -- SoundCloud confusingly calls this
    /// "username" but it's a free-text display name, e.g. "wockk").
    pub username: Option<String>,
    pub avatar_url: Option<String>,
    /// The actual @handle / URL slug, e.g. "rudolf" for soundcloud.com/rudolf.
    pub permalink: Option<String>,
    pub permalink_url: Option<String>,
}

/// Full profile -- richer than the embedded `User` on a track/playlist.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Profile {
    pub id: i64,
    /// Display name (NOT the @handle -- see User::username).
    pub username: Option<String>,
    pub full_name: Option<String>,
    pub avatar_url: Option<String>,
    /// The actual @handle / URL slug, e.g. "rudolf" for soundcloud.com/rudolf.
    pub permalink: Option<String>,
    pub permalink_url: Option<String>,
    pub description: Option<String>,
    pub city: Option<String>,
    pub country_code: Option<String>,
    pub followers_count: Option<i64>,
    pub followings_count: Option<i64>,
    pub track_count: Option<i64>,
    pub visuals: Option<Visuals>,
}

impl Profile {
    /// The user's profile banner image, if they've set one.
    pub fn banner_url(&self) -> Option<&str> {
        self.visuals.as_ref()?.visuals.first()?.visual_url.as_deref()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Visuals {
    #[serde(default)]
    pub visuals: Vec<VisualItem>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VisualItem {
    pub visual_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchUsersResponse {
    #[serde(default)]
    pub collection: Vec<Profile>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FollowersResponse {
    #[serde(default)]
    pub collection: Vec<Profile>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserCommentsResponse {
    #[serde(default)]
    pub collection: Vec<UserComment>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserComment {
    pub id: i64,
    pub body: Option<String>,
    pub created_at: Option<String>,
    pub track_id: Option<i64>,
    /// Not present in the raw API response -- filled in afterward by
    /// looking up each referenced track, since the comment endpoint only
    /// gives a bare track_id.
    #[serde(skip_deserializing)]
    pub track_title: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TranscodingFormat {
    pub protocol: Option<String>,
    pub mime_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Transcoding {
    pub url: String,
    pub format: Option<TranscodingFormat>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Media {
    #[serde(default)]
    pub transcodings: Vec<Transcoding>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Track {
    pub id: i64,
    pub title: Option<String>,
    pub permalink_url: Option<String>,
    pub artwork_url: Option<String>,
    pub duration: Option<i64>,
    pub genre: Option<String>,
    pub user: Option<User>,
    #[serde(default)]
    pub media: Media,
    #[serde(default)]
    pub streamable: bool,
    pub policy: Option<String>,
    pub likes_count: Option<i64>,
    pub reposts_count: Option<i64>,
    pub comment_count: Option<i64>,
    pub playback_count: Option<i64>,
    pub created_at: Option<String>,
}

impl Track {
    /// Transcodings this app can actually play, in priority order:
    /// 1. progressive (a plain, directly playable MP3).
    /// 2. plain (unencrypted) HLS with raw MP3 segments -- these concatenate
    ///    into one valid MP3 stream since MP3 frames are self-synchronizing.
    /// Deliberately excludes anything else, since on many monetized/label
    /// tracks that's `cbc-encrypted-hls`/`ctr-encrypted-hls` (real DRM) or
    /// AAC-in-fMP4 HLS, neither of which this app can play.
    ///
    /// More than one is returned (not just the "best" one) because SoundCloud
    /// sometimes *lists* a progressive transcoding for a monetized track that
    /// 404s when actually resolved without full user auth -- the caller
    /// should try each in order and fall through on failure rather than
    /// giving up after the first.
    ///
    /// If the track has ANY encrypted-HLS transcoding listed at all, this
    /// returns empty even if progressive/plain-HLS are *also* listed --
    /// confirmed live (with a real logged-in oauth_token, not just an
    /// anonymous request) that on such tracks the plain listings are
    /// vestigial and 404 when actually resolved. Presence of an encrypted
    /// variant is a reliable signal the track is real DRM (Apple FairPlay /
    /// Widevine), which only SoundCloud's own licensed client can decrypt.
    pub fn candidate_transcodings(&self) -> Vec<&Transcoding> {
        let has_drm = self.media.transcodings.iter().any(|t| {
            t.format
                .as_ref()
                .and_then(|f| f.protocol.as_deref())
                .is_some_and(|p| p.contains("encrypted"))
        });
        if has_drm {
            return Vec::new();
        }

        let mut progressive: Vec<&Transcoding> = self
            .media
            .transcodings
            .iter()
            .filter(|t| t.format.as_ref().and_then(|f| f.protocol.as_deref()) == Some("progressive"))
            .collect();
        let hls_mp3: Vec<&Transcoding> = self
            .media
            .transcodings
            .iter()
            .filter(|t| {
                let format = t.format.as_ref();
                format.and_then(|f| f.protocol.as_deref()) == Some("hls")
                    && format.and_then(|f| f.mime_type.as_deref()) == Some("audio/mpeg")
            })
            .collect();
        progressive.extend(hls_mp3);
        progressive
    }

    /// True if this track has no playable transcoding for us but does have
    /// media available (i.e. it's DRM-protected or an unsupported codec),
    /// so we can report a specific, honest error instead of a generic one.
    pub fn only_has_unsupported_transcodings(&self) -> bool {
        self.candidate_transcodings().is_empty() && !self.media.transcodings.is_empty()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Playlist {
    pub id: i64,
    pub title: Option<String>,
    pub permalink_url: Option<String>,
    pub artwork_url: Option<String>,
    pub track_count: Option<i64>,
    #[serde(default)]
    pub tracks: Vec<Track>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchTracksResponse {
    #[serde(default)]
    pub collection: Vec<Track>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LikesResponse {
    #[serde(default)]
    pub collection: Vec<TrackLike>,
    pub next_href: Option<String>,
}

/// Some like-listing endpoints wrap each track in a `{ track: {...} }`
/// envelope, others return the track object directly -- accept either.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TrackLike {
    Wrapped { track: Track },
    Bare(Track),
}

impl TrackLike {
    pub fn into_track(self) -> Track {
        match self {
            TrackLike::Wrapped { track } => track,
            TrackLike::Bare(track) => track,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlaylistsResponse {
    #[serde(default)]
    pub collection: Vec<PlaylistLike>,
}

/// The liked_and_owned endpoint wraps each item as `{ playlist: {...} }`
/// (mirroring track_likes), but some variants return the playlist bare --
/// accept either rather than assume one shape.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PlaylistLike {
    Wrapped { playlist: Playlist },
    Bare(Playlist),
}

impl PlaylistLike {
    pub fn into_playlist(self) -> Playlist {
        match self {
            PlaylistLike::Wrapped { playlist } => playlist,
            PlaylistLike::Bare(playlist) => playlist,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StreamResolution {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserTracksResponse {
    #[serde(default)]
    pub collection: Vec<Track>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Comment {
    pub id: i64,
    pub body: Option<String>,
    pub created_at: Option<String>,
    pub user: Option<User>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommentsResponse {
    #[serde(default)]
    pub collection: Vec<Comment>,
    pub next_href: Option<String>,
}

/// The reposts list mixes track and playlist reposts; only track reposts
/// are surfaced today. Shape mirrors /stream items.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RepostsResponse {
    #[serde(default)]
    pub collection: Vec<FeedItem>,
    pub next_href: Option<String>,
}

/// /stream items are heterogeneous (track upload, track repost, playlist
/// upload, playlist repost...). We only care about the ones that carry a
/// playable track, wherever it happens to live in the item shape -- SoundCloud
/// nests it directly as `track` for uploads and under `origin` for reposts.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FeedResponse {
    #[serde(default)]
    pub collection: Vec<FeedItem>,
    pub next_href: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FeedItem {
    #[serde(rename = "type")]
    pub kind: Option<String>,
    pub track: Option<Track>,
    pub origin: Option<serde_json::Value>,
}

impl FeedItem {
    /// Best-effort extraction of a playable track from whichever shape this
    /// particular feed item turned out to have.
    pub fn extract_track(&self) -> Option<Track> {
        if let Some(t) = &self.track {
            return Some(t.clone());
        }
        let origin = self.origin.as_ref()?;
        if let Ok(t) = serde_json::from_value::<Track>(origin.clone()) {
            return Some(t);
        }
        serde_json::from_value::<Track>(origin.get("track")?.clone()).ok()
    }
}
