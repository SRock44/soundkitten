//! Verifies tolerant deserialization against saved fixture JSON, so schema
//! drift (SoundCloud adding new fields, omitting optional ones) doesn't
//! break parsing. No live network needed -- safe to run in CI.

use app_lib::soundcloud::models::{LikesResponse, PlaylistsResponse, SearchTracksResponse};

#[test]
fn parses_search_tracks_with_unknown_and_missing_fields() {
    let raw = include_str!("fixtures/search_tracks.json");
    let parsed: SearchTracksResponse = serde_json::from_str(raw).expect("should parse despite extra/missing fields");

    assert_eq!(parsed.collection.len(), 2);

    let full = &parsed.collection[0];
    assert_eq!(full.id, 123456789);
    assert_eq!(full.title.as_deref(), Some("Example Track"));
    let transcoding = full.candidate_transcodings();
    assert!(transcoding.first().expect("progressive transcoding should be picked").url.contains("progressive"));

    let sparse = &parsed.collection[1];
    assert_eq!(sparse.id, 42);
    assert!(sparse.title.is_some());
    assert!(sparse.user.is_none());
    assert!(sparse.candidate_transcodings().is_empty());
}

#[test]
fn parses_likes_envelope_and_missing_optional_track_fields() {
    let raw = include_str!("fixtures/likes.json");
    let parsed: LikesResponse = serde_json::from_str(raw).expect("should parse likes envelope");

    assert_eq!(parsed.collection.len(), 2);
    let first = parsed.collection[0].clone().into_track();
    let second = parsed.collection[1].clone().into_track();
    assert_eq!(first.id, 111);
    assert_eq!(second.user.as_ref().unwrap().username.as_deref(), Some("artist-two"));
}

#[test]
fn parses_bare_track_likes_without_wrapper_envelope() {
    use app_lib::soundcloud::models::LikesResponse;
    let raw = r#"{ "collection": [ { "id": 999, "title": "Bare Track" } ] }"#;
    let parsed: LikesResponse = serde_json::from_str(raw).expect("should parse bare-track shape");
    assert_eq!(parsed.collection[0].clone().into_track().id, 999);
}

#[test]
fn parses_playlists_with_and_without_tracks_field() {
    let raw = include_str!("fixtures/playlists.json");
    let parsed: PlaylistsResponse = serde_json::from_str(raw).expect("should parse playlists");

    assert_eq!(parsed.collection.len(), 2);
    let first = parsed.collection[0].clone().into_playlist();
    let second = parsed.collection[1].clone().into_playlist();
    assert_eq!(first.tracks.len(), 2);
    // playlist 556 is a bare (unwrapped) entry that omits "tracks" entirely --
    // must default to empty and still parse via the untagged Bare variant.
    assert!(second.tracks.is_empty());
    assert_eq!(second.id, 556);
}
