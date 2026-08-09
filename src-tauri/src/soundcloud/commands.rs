//! Tauri command surface for the frontend. Stream resolution is intentionally
//! not exposed here -- it's used internally by the sc-stream:// proxy (Phase 4)
//! so a raw, time-limited CDN URL never has to round-trip through the frontend.

use super::api;
use super::models::{Comment, Playlist, Profile, Selection, Track, UserComment};
use crate::auth::get_stored_token;

fn require_token() -> Result<String, String> {
    get_stored_token().ok_or_else(|| "not logged in".to_string())
}

fn http_client() -> reqwest::Client {
    reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) soundcloud-desktop/0.1")
        .build()
        .expect("failed to build reqwest client")
}

#[tauri::command]
pub async fn sc_search(query: String) -> Result<Vec<Track>, String> {
    let client = http_client();
    api::search_tracks(&client, &query, get_stored_token().as_deref())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sc_resolve(url: String) -> Result<Track, String> {
    let client = http_client();
    api::resolve_track(&client, &url, get_stored_token().as_deref())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sc_likes() -> Result<Vec<Track>, String> {
    let Some(token) = get_stored_token() else {
        return Err("not logged in".into());
    };
    let client = http_client();
    api::get_likes(&client, &token).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sc_playlists() -> Result<Vec<Playlist>, String> {
    let Some(token) = get_stored_token() else {
        return Err("not logged in".into());
    };
    let client = http_client();
    api::get_playlists(&client, &token).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sc_me() -> Result<Profile, String> {
    let Some(token) = get_stored_token() else {
        return Err("not logged in".into());
    };
    let client = http_client();
    api::get_me(&client, &token).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sc_user_profile(user_id: i64) -> Result<Profile, String> {
    let client = http_client();
    api::get_user(&client, user_id, get_stored_token().as_deref())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sc_user_tracks(user_id: i64) -> Result<Vec<Track>, String> {
    let client = http_client();
    api::get_user_tracks(&client, user_id, get_stored_token().as_deref())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sc_search_users(query: String) -> Result<Vec<Profile>, String> {
    let client = http_client();
    api::search_users(&client, &query, get_stored_token().as_deref())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sc_feed() -> Result<Vec<Track>, String> {
    let Some(token) = get_stored_token() else {
        return Err("not logged in".into());
    };
    let client = http_client();
    api::get_feed(&client, &token).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sc_like_track(track_id: i64) -> Result<(), String> {
    let token = require_token()?;
    api::like_track(&http_client(), track_id, &token).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sc_unlike_track(track_id: i64) -> Result<(), String> {
    let token = require_token()?;
    api::unlike_track(&http_client(), track_id, &token).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sc_repost_track(track_id: i64) -> Result<(), String> {
    let token = require_token()?;
    api::repost_track(&http_client(), track_id, &token).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sc_unrepost_track(track_id: i64) -> Result<(), String> {
    let token = require_token()?;
    api::unrepost_track(&http_client(), track_id, &token).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sc_track_comments(track_id: i64) -> Result<Vec<Comment>, String> {
    let client = http_client();
    api::get_track_comments(&client, track_id, get_stored_token().as_deref())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sc_post_comment(track_id: i64, body: String) -> Result<Comment, String> {
    let token = require_token()?;
    api::post_comment(&http_client(), track_id, &body, &token).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sc_user_reposts(user_id: i64) -> Result<Vec<Track>, String> {
    let client = http_client();
    api::get_user_reposts(&client, user_id, get_stored_token().as_deref())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sc_user_playlists(user_id: i64) -> Result<Vec<Playlist>, String> {
    let client = http_client();
    api::get_user_playlists(&client, user_id, get_stored_token().as_deref())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sc_mixed_selections() -> Result<Vec<Selection>, String> {
    let client = http_client();
    api::get_mixed_selections(&client, get_stored_token().as_deref())
        .await
        .map_err(|e| e.to_string())
}

/// Hydrates a SoundCloud-generated system playlist ("Your Mix N", etc) by
/// batch-fetching full track data for its id stubs.
#[tauri::command]
pub async fn sc_system_playlist_tracks(track_ids: Vec<i64>) -> Result<Vec<Track>, String> {
    let client = http_client();
    api::get_tracks_by_ids(&client, &track_ids, get_stored_token().as_deref())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sc_playlist(playlist_id: i64) -> Result<Playlist, String> {
    let client = http_client();
    api::get_playlist(&client, playlist_id, get_stored_token().as_deref())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sc_user_followers(user_id: i64) -> Result<Vec<Profile>, String> {
    let client = http_client();
    api::get_user_followers(&client, user_id, get_stored_token().as_deref())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sc_user_followings(user_id: i64) -> Result<Vec<Profile>, String> {
    let client = http_client();
    api::get_user_followings(&client, user_id, get_stored_token().as_deref())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sc_my_followings_ids() -> Result<Vec<i64>, String> {
    let token = require_token()?;
    api::get_my_followings_ids(&http_client(), &token).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sc_user_comments(user_id: i64) -> Result<Vec<UserComment>, String> {
    let client = http_client();
    api::get_user_comments(&client, user_id, get_stored_token().as_deref())
        .await
        .map_err(|e| e.to_string())
}
