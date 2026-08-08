//! Typed, tolerant models for the unofficial SoundCloud v2 API.
//! All fields beyond the ones we actually use are `Option` or ignored,
//! so schema drift (SoundCloud adding/removing fields) doesn't break parsing.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub username: Option<String>,
    pub avatar_url: Option<String>,
    pub permalink_url: Option<String>,
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
}

impl Track {
    /// Picks the progressive-MP3 transcoding if present, else the first available.
    pub fn preferred_transcoding(&self) -> Option<&Transcoding> {
        self.media
            .transcodings
            .iter()
            .find(|t| t.format.as_ref().and_then(|f| f.protocol.as_deref()) == Some("progressive"))
            .or_else(|| self.media.transcodings.first())
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
pub struct Me {
    pub id: i64,
    pub username: Option<String>,
}
