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
    /// Used to cheaply detect "did my likes change" (e.g. after liking a
    /// track on soundcloud.com in a browser) without re-fetching the whole
    /// likes list on a timer -- see the frontend's likes-sync logic.
    pub likes_count: Option<i64>,
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

/// GET /users/{id}/followings/ids -- a lightweight id-only listing (no
/// per-follow Profile hydration) confirmed live via SoundCloud's own web
/// bundle, which uses this exact route for the same purpose we do: checking
/// "do I follow this person" cheaply, without needing a boolean field on
/// `/users/{id}` itself (it doesn't have one).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FollowingIdsResponse {
    #[serde(default)]
    pub collection: Vec<i64>,
    pub next_href: Option<String>,
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
    /// Points to a small public, CORS-open JSON file on wave.sndcdn.com
    /// ({width, height, samples: [...]}) -- SoundCloud's own precomputed
    /// per-track amplitude envelope, the same data their own player draws
    /// as the orange waveform. Confirmed live (real API response): no
    /// client_id or auth needed to fetch it, so the frontend fetches it
    /// directly (see PlayerBar.svelte's loadWaveform) rather than needing
    /// a Rust-side proxy.
    pub waveform_url: Option<String>,
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
    /// The playlist's owner. `None` for playlists sourced from an endpoint
    /// that doesn't embed it -- absence doesn't mean "not owned by anyone",
    /// callers gating owner-only controls should treat `None` as "unknown,
    /// don't show owner controls" rather than "not owned."
    pub user: Option<User>,
}

/// SoundCloud's generated/algorithmic sets ("Your Mix N", "Related tracks:
/// ...", weekly mood mixes, etc), as returned inside `/mixed-selections`.
/// These are NOT real playlists -- their `id` is a string urn like
/// `soundcloud:system-playlists:your-moods:{userId}:1`, not an integer, and
/// there is no `/playlists/{id}` resource for them (confirmed live: that
/// 404s). Their `tracks` array only contains id stubs, not full track data,
/// so hydrating one requires a separate batch `/tracks?ids=...` call.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SystemPlaylist {
    pub id: String,
    pub title: Option<String>,
    pub artwork_url: Option<String>,
    pub calculated_artwork_url: Option<String>,
    pub permalink_url: Option<String>,
    #[serde(default)]
    pub tracks: Vec<TrackStub>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TrackStub {
    pub id: i64,
}

/// An item inside a mixed-selections module: either a real playlist or a
/// SoundCloud-generated system playlist. Modules like "Recently Played" mix
/// in bare `"kind":"user"` entries too (confirmed live) -- those, and any
/// other kind we don't render, are dropped during parsing rather than
/// misrouted, since a bare user object structurally happens to satisfy
/// `Playlist`'s all-optional-except-id shape (its numeric `id` parses fine,
/// yielding a title-less, artwork-less "ghost" card) if matched blindly.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum SelectionEntry {
    Playlist(Playlist),
    SystemPlaylist(SystemPlaylist),
}

impl SelectionEntry {
    fn from_value(v: serde_json::Value) -> Option<Self> {
        match v.get("kind").and_then(|k| k.as_str())? {
            "playlist" => serde_json::from_value::<Playlist>(v).ok().map(SelectionEntry::Playlist),
            "system-playlist" => serde_json::from_value::<SystemPlaylist>(v).ok().map(SelectionEntry::SystemPlaylist),
            _ => None,
        }
    }

    fn id_key(&self) -> String {
        match self {
            SelectionEntry::Playlist(p) => p.id.to_string(),
            SelectionEntry::SystemPlaylist(sp) => sp.id.clone(),
        }
    }

    fn has_title(&self) -> bool {
        let title = match self {
            SelectionEntry::Playlist(p) => &p.title,
            SelectionEntry::SystemPlaylist(sp) => &sp.title,
        };
        title.as_deref().is_some_and(|t| !t.trim().is_empty())
    }
}

/// SoundCloud's homepage curation feed -- "Trending by genre", "Artists to
/// watch out for", "Curated by SoundCloud", etc. Confirmed live via
/// GET /mixed-selections (note: hyphen, not underscore -- the underscore
/// spelling used by older third-party write-ups 404s).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MixedSelectionsResponse {
    #[serde(default, deserialize_with = "deserialize_selections_lenient")]
    pub collection: Vec<Selection>,
}

fn deserialize_selections_lenient<'de, D>(deserializer: D) -> Result<Vec<Selection>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let raw: Vec<serde_json::Value> = Vec::deserialize(deserializer)?;
    Ok(raw
        .into_iter()
        .filter_map(|v| serde_json::from_value::<Selection>(v).ok())
        .collect())
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Selection {
    pub urn: Option<String>,
    pub title: Option<String>,
    #[serde(default)]
    pub items: SelectionItems,
}

impl Selection {
    /// Drops title-less entries (e.g. the bare "kind":"user" objects
    /// SoundCloud mixes into modules like "Recently Played" -- confirmed
    /// live) and de-duplicates by id, since the API itself repeats the same
    /// system playlist multiple times in that module (once per play event,
    /// it seems) rather than us double-parsing anything.
    pub fn cleaned(mut self) -> Self {
        let mut seen = std::collections::HashSet::new();
        self.items.collection.retain(|entry| entry.has_title() && seen.insert(entry.id_key()));
        self
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct SelectionItems {
    /// Deserialized permissively: some selection modules mix in items that
    /// aren't playlist- or system-playlist-shaped (bare user/artist entries,
    /// etc). Parsing the whole array strictly would fail the entire response
    /// over one odd item, so parse each item on its own and drop the ones
    /// that don't fit either shape.
    #[serde(default, deserialize_with = "deserialize_selection_entries_lenient")]
    pub collection: Vec<SelectionEntry>,
}

fn deserialize_selection_entries_lenient<'de, D>(deserializer: D) -> Result<Vec<SelectionEntry>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let raw: Vec<serde_json::Value> = Vec::deserialize(deserializer)?;
    Ok(raw.into_iter().filter_map(SelectionEntry::from_value).collect())
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchTracksResponse {
    #[serde(default)]
    pub collection: Vec<Track>,
}

/// An item from the unified GET /search endpoint (as opposed to the
/// type-specific /search/tracks and /search/users this app already used) --
/// confirmed live: it returns tracks, users, and playlists interleaved in
/// relevance order, each carrying a "kind" field. Field shapes per kind are
/// identical to what Track/Profile/Playlist already model (same API
/// family, different route), so no new struct fields needed, just this
/// kind-tagged wrapper -- dispatch on decode mirrors SelectionEntry above,
/// but unlike Playlist/SystemPlaylist (distinguishable by id type alone,
/// string vs number), Track/Profile/Playlist all have plain numeric ids
/// with no structurally distinguishing field -- so, unlike SelectionEntry,
/// this MUST serialize with an explicit "kind" tag or the frontend has no
/// way to tell the variants apart at runtime.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum SearchResultItem {
    Track(Track),
    User(Profile),
    Playlist(Playlist),
}

impl SearchResultItem {
    fn from_value(v: serde_json::Value) -> Option<Self> {
        match v.get("kind").and_then(|k| k.as_str())? {
            "track" => serde_json::from_value::<Track>(v).ok().map(SearchResultItem::Track),
            "user" => serde_json::from_value::<Profile>(v).ok().map(SearchResultItem::User),
            "playlist" => serde_json::from_value::<Playlist>(v).ok().map(SearchResultItem::Playlist),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchAllResponse {
    #[serde(default, deserialize_with = "deserialize_search_items_lenient")]
    pub collection: Vec<SearchResultItem>,
}

fn deserialize_search_items_lenient<'de, D>(deserializer: D) -> Result<Vec<SearchResultItem>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let raw: Vec<serde_json::Value> = Vec::deserialize(deserializer)?;
    Ok(raw.into_iter().filter_map(SearchResultItem::from_value).collect())
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
