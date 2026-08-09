//! Typed API operations built on top of soundcloud::authed_get.

use super::models::{
    Comment, CommentsResponse, FeedItem, FeedResponse, FollowersResponse, FollowingIdsResponse, LikesResponse,
    MixedSelectionsResponse, Playlist, PlaylistsResponse, Profile, RepostsResponse, SearchTracksResponse,
    SearchUsersResponse, StreamResolution, Track, UserComment, UserCommentsResponse, UserTracksResponse,
};
use super::{authed_delete, authed_get, authed_post, authed_put, get_full_url, resolve_raw};

pub async fn get_me(client: &reqwest::Client, oauth_token: &str) -> anyhow::Result<Profile> {
    authed_get(client, "/me", &[], Some(oauth_token)).await
}

/// Current user id -- likes/playlists are fetched from `/users/{id}/...`, not `/me/...`.
async fn current_user_id(client: &reqwest::Client, oauth_token: &str) -> anyhow::Result<i64> {
    Ok(get_me(client, oauth_token).await?.id)
}

pub async fn like_track(client: &reqwest::Client, track_id: i64, oauth_token: &str) -> anyhow::Result<()> {
    let user_id = current_user_id(client, oauth_token).await?;
    authed_put(client, &format!("/users/{user_id}/track_likes/{track_id}"), oauth_token).await
}

pub async fn unlike_track(client: &reqwest::Client, track_id: i64, oauth_token: &str) -> anyhow::Result<()> {
    let user_id = current_user_id(client, oauth_token).await?;
    authed_delete(client, &format!("/users/{user_id}/track_likes/{track_id}"), oauth_token).await
}

/// NOTE: unlike likes (`/users/{id}/track_likes/{id}`), reposts write via
/// `/me/track_reposts/{id}` -- confirmed by searching SoundCloud's own web
/// app bundle for the literal route string, after `/users/{id}/track_reposts/{id}`
/// (the pattern that would've matched likes) turned out to reliably 404.
pub async fn repost_track(client: &reqwest::Client, track_id: i64, oauth_token: &str) -> anyhow::Result<()> {
    authed_put(client, &format!("/me/track_reposts/{track_id}"), oauth_token).await
}

pub async fn unrepost_track(client: &reqwest::Client, track_id: i64, oauth_token: &str) -> anyhow::Result<()> {
    authed_delete(client, &format!("/me/track_reposts/{track_id}"), oauth_token).await
}

pub async fn get_track_comments(client: &reqwest::Client, track_id: i64, oauth_token: Option<&str>) -> anyhow::Result<Vec<Comment>> {
    let path = format!("/tracks/{track_id}/comments");
    let resp: CommentsResponse =
        authed_get(client, &path, &[("threaded", "0"), ("filter_replies", "0"), ("limit", "50")], oauth_token).await?;
    Ok(resp.collection)
}

pub async fn post_comment(client: &reqwest::Client, track_id: i64, body: &str, oauth_token: &str) -> anyhow::Result<Comment> {
    let path = format!("/tracks/{track_id}/comments");
    let payload = serde_json::json!({ "comment": { "body": body } });
    authed_post(client, &path, &payload, oauth_token).await
}

/// A user's reposted tracks (track reposts only -- playlist reposts are
/// skipped). NOTE: this is NOT at /users/{id}/track_reposts (that 404s
/// despite mirroring the track_likes path pattern) -- reposts are only
/// exposed via the per-user stream endpoint, confirmed live against a real
/// account with actual reposts.
pub async fn get_user_reposts(client: &reqwest::Client, user_id: i64, oauth_token: Option<&str>) -> anyhow::Result<Vec<Track>> {
    let path = format!("/stream/users/{user_id}/reposts");
    let resp: RepostsResponse = authed_get(client, &path, &[("limit", "50")], oauth_token).await?;
    Ok(resp.collection.iter().filter_map(FeedItem::extract_track).collect())
}

/// The user's own public playlists (not their liked-and-owned collection).
pub async fn get_user_playlists(client: &reqwest::Client, user_id: i64, oauth_token: Option<&str>) -> anyhow::Result<Vec<Playlist>> {
    let path = format!("/users/{user_id}/playlists");
    let resp: PlaylistsResponse = authed_get(client, &path, &[("limit", "50")], oauth_token).await?;
    Ok(resp.collection.into_iter().map(|p| p.into_playlist()).collect())
}

/// Playlist listing endpoints (likes, own playlists) only return metadata +
/// track_count, NOT the actual tracks (confirmed live: tracks.len() == 0
/// despite a nonzero track_count) -- this fetches the full, hydrated
/// playlist with its track list when the user actually opens one.
/// SoundCloud's homepage curation modules ("Trending by genre", "Artists to
/// watch out for", "Curated by SoundCloud"). Works without auth, but pass
/// the token when available since results may be personalized by account.
pub async fn get_mixed_selections(client: &reqwest::Client, oauth_token: Option<&str>) -> anyhow::Result<Vec<super::models::Selection>> {
    let resp: MixedSelectionsResponse = authed_get(client, "/mixed-selections", &[], oauth_token).await?;
    Ok(resp.collection.into_iter().map(|s| s.cleaned()).collect())
}

pub async fn get_playlist(client: &reqwest::Client, playlist_id: i64, oauth_token: Option<&str>) -> anyhow::Result<Playlist> {
    let path = format!("/playlists/{playlist_id}");
    authed_get(client, &path, &[], oauth_token).await
}

/// Batch-hydrates full track objects for a set of ids. Used for system
/// playlists (see [`super::models::SystemPlaylist`]) whose embedded `tracks`
/// array only contains id stubs, not full track data.
pub async fn get_tracks_by_ids(client: &reqwest::Client, ids: &[i64], oauth_token: Option<&str>) -> anyhow::Result<Vec<Track>> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    let ids_str = ids.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    authed_get(client, "/tracks", &[("ids", ids_str.as_str())], oauth_token).await
}

pub async fn get_user_followers(client: &reqwest::Client, user_id: i64, oauth_token: Option<&str>) -> anyhow::Result<Vec<Profile>> {
    let path = format!("/users/{user_id}/followers");
    let resp: FollowersResponse = authed_get(client, &path, &[("limit", "50")], oauth_token).await?;
    Ok(resp.collection)
}

/// All ids the current user follows, used to compute follow-state per
/// profile client-side. Follow/unfollow *writes* are blocked behind
/// SoundCloud's DataDome bot-protection (confirmed live: `POST
/// /me/followings/:id` -- the correct route, per the web bundle --
/// consistently 403s with a CAPTCHA challenge even with a valid oauth
/// token), so this app can only reflect real follow state, not change it;
/// the UI sends users to soundcloud.com to actually follow someone.
pub async fn get_my_followings_ids(client: &reqwest::Client, oauth_token: &str) -> anyhow::Result<Vec<i64>> {
    let user_id = current_user_id(client, oauth_token).await?;
    let path = format!("/users/{user_id}/followings/ids");

    let mut all_ids = Vec::new();
    let mut next_url: Option<String> = None;
    for _page in 0..50 {
        let resp: FollowingIdsResponse = match &next_url {
            Some(url) => get_full_url(client, url, Some(oauth_token)).await?,
            None => authed_get(client, &path, &[("limit", "5000")], Some(oauth_token)).await?,
        };
        all_ids.extend(resp.collection);
        match resp.next_href {
            Some(href) => next_url = Some(href),
            None => break,
        }
    }
    Ok(all_ids)
}

pub async fn get_user_followings(client: &reqwest::Client, user_id: i64, oauth_token: Option<&str>) -> anyhow::Result<Vec<Profile>> {
    let path = format!("/users/{user_id}/followings");
    let resp: FollowersResponse = authed_get(client, &path, &[("limit", "50")], oauth_token).await?;
    Ok(resp.collection)
}

/// A user's own posted comments, with each comment's bare `track_id` resolved
/// into the track's title (the comments endpoint doesn't include it directly).
/// Capped at 5 to bound the extra per-comment track lookups.
pub async fn get_user_comments(client: &reqwest::Client, user_id: i64, oauth_token: Option<&str>) -> anyhow::Result<Vec<UserComment>> {
    let path = format!("/users/{user_id}/comments");
    let resp: UserCommentsResponse = authed_get(client, &path, &[("limit", "5")], oauth_token).await?;

    let mut comments = resp.collection;
    for comment in &mut comments {
        if let Some(track_id) = comment.track_id {
            if let Ok(track) = get_track(client, track_id, oauth_token).await {
                comment.track_title = track.title;
            }
        }
    }
    Ok(comments)
}

pub async fn get_user(client: &reqwest::Client, user_id: i64, oauth_token: Option<&str>) -> anyhow::Result<Profile> {
    let path = format!("/users/{user_id}");
    authed_get(client, &path, &[], oauth_token).await
}

pub async fn get_user_tracks(client: &reqwest::Client, user_id: i64, oauth_token: Option<&str>) -> anyhow::Result<Vec<Track>> {
    let path = format!("/users/{user_id}/tracks");
    let resp: UserTracksResponse = authed_get(client, &path, &[("limit", "50")], oauth_token).await?;
    Ok(resp.collection)
}

pub async fn search_users(client: &reqwest::Client, query: &str, oauth_token: Option<&str>) -> anyhow::Result<Vec<Profile>> {
    let resp: SearchUsersResponse =
        authed_get(client, "/search/users", &[("q", query), ("limit", "20")], oauth_token).await?;
    Ok(resp.collection)
}

/// Personalized activity stream (new uploads/reposts from people you follow) --
/// this is what backs SoundCloud's own "Feed"/Home. Only the first page is
/// fetched since it's a preview list, not a fully paginated view.
pub async fn get_feed(client: &reqwest::Client, oauth_token: &str) -> anyhow::Result<Vec<Track>> {
    let resp: FeedResponse = authed_get(client, "/stream", &[("limit", "30")], Some(oauth_token)).await?;
    Ok(resp.collection.iter().filter_map(FeedItem::extract_track).collect())
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

/// Follows `next_href` cursors to fetch the user's *entire* like list --
/// SoundCloud paginates at ~48-50 tracks per page, and users can easily have
/// hundreds or thousands of likes. Capped at 100 pages (~5000 tracks) as a
/// sanity limit against a runaway pagination loop.
pub async fn get_likes(client: &reqwest::Client, oauth_token: &str) -> anyhow::Result<Vec<Track>> {
    let user_id = current_user_id(client, oauth_token).await?;
    let path = format!("/users/{user_id}/track_likes");

    let mut all_tracks = Vec::new();
    let mut next_url: Option<String> = None;
    for _page in 0..100 {
        let resp: LikesResponse = match &next_url {
            Some(url) => get_full_url(client, url, Some(oauth_token)).await?,
            None => authed_get(client, &path, &[("limit", "200")], Some(oauth_token)).await?,
        };
        all_tracks.extend(resp.collection.into_iter().map(|l| l.into_track()));
        match resp.next_href {
            Some(href) => next_url = Some(href),
            None => break,
        }
    }
    Ok(all_tracks)
}

pub async fn get_playlists(client: &reqwest::Client, oauth_token: &str) -> anyhow::Result<Vec<Playlist>> {
    let user_id = current_user_id(client, oauth_token).await?;
    let path = format!("/users/{user_id}/playlists/liked_and_owned");
    let resp: PlaylistsResponse = authed_get(client, &path, &[("limit", "50")], Some(oauth_token)).await?;
    Ok(resp.collection.into_iter().map(|p| p.into_playlist()).collect())
}

/// Resolves a specific transcoding's descriptor URL into a final,
/// time-limited playable stream URL. Callers with multiple candidate
/// transcodings (see Track::candidate_transcodings) should try each in turn
/// on failure -- SoundCloud sometimes *lists* a transcoding that 404s when
/// actually resolved without full user auth.
pub async fn resolve_transcoding_url(client: &reqwest::Client, transcoding: &super::models::Transcoding, oauth_token: Option<&str>) -> anyhow::Result<String> {
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
