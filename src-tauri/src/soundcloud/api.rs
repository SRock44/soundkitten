//! Typed API operations built on top of soundcloud::authed_get.

use super::models::{
    LikesResponse, Me, Playlist, PlaylistsResponse, SearchTracksResponse, StreamResolution, Track,
};
use super::{authed_get, resolve_raw};

/// Current user id -- likes/playlists are fetched from `/users/{id}/...`, not `/me/...`.
async fn current_user_id(client: &reqwest::Client, oauth_token: &str) -> anyhow::Result<i64> {
    let me: Me = authed_get(client, "/me", &[], Some(oauth_token)).await?;
    Ok(me.id)
}

pub async fn search_tracks(client: &reqwest::Client, query: &str, oauth_token: Option<&str>) -> anyhow::Result<Vec<Track>> {
    let resp: SearchTracksResponse =
        authed_get(client, "/search/tracks", &[("q", query), ("limit", "20")], oauth_token).await?;
    Ok(resp.collection)
}

pub async fn resolve_track(client: &reqwest::Client, url: &str, oauth_token: Option<&str>) -> anyhow::Result<Track> {
    resolve_raw(client, url, oauth_token).await
}

pub async fn get_track(client: &reqwest::Client, track_id: i64, oauth_token: Option<&str>) -> anyhow::Result<Track> {
    let path = format!("/tracks/{track_id}");
    authed_get(client, &path, &[], oauth_token).await
}

pub async fn get_likes(client: &reqwest::Client, oauth_token: &str) -> anyhow::Result<Vec<Track>> {
    let user_id = current_user_id(client, oauth_token).await?;
    let path = format!("/users/{user_id}/track_likes");
    let resp: LikesResponse = authed_get(client, &path, &[("limit", "50")], Some(oauth_token)).await?;
    Ok(resp.collection.into_iter().map(|l| l.into_track()).collect())
}

pub async fn get_playlists(client: &reqwest::Client, oauth_token: &str) -> anyhow::Result<Vec<Playlist>> {
    let user_id = current_user_id(client, oauth_token).await?;
    let path = format!("/users/{user_id}/playlists/liked_and_owned");
    let resp: PlaylistsResponse = authed_get(client, &path, &[("limit", "50")], Some(oauth_token)).await?;
    Ok(resp.collection.into_iter().map(|p| p.into_playlist()).collect())
}

/// Resolves a track's transcoding URL into a final, time-limited playable stream URL.
pub async fn resolve_stream_url(client: &reqwest::Client, track: &Track, oauth_token: Option<&str>) -> anyhow::Result<String> {
    let transcoding = track
        .preferred_transcoding()
        .ok_or_else(|| anyhow::anyhow!("track {} has no available transcodings (may be geo-blocked, private, or removed)", track.id))?;

    let client_id = super::get_client_id(client).await?;
    let mut req = client.get(&transcoding.url).query(&[("client_id", client_id.as_str())]);
    if let Some(token) = oauth_token {
        req = req.header("Authorization", format!("OAuth {token}"));
    }
    let resp = req.send().await?;
    let status = resp.status();
    let text = resp.text().await?;
    if !status.is_success() {
        anyhow::bail!("stream resolution returned {status}: {}", if text.is_empty() { "<empty body>" } else { &text });
    }
    let resolved: StreamResolution = serde_json::from_str(&text)
        .map_err(|e| anyhow::anyhow!("stream resolution returned {status} but body didn't match the expected shape: {e}"))?;
    Ok(resolved.url)
}
