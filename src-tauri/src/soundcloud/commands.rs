//! Tauri command surface for the frontend. Stream resolution is intentionally
//! not exposed here -- it's used internally by the sc-stream:// proxy (Phase 4)
//! so a raw, time-limited CDN URL never has to round-trip through the frontend.

use super::api;
use super::models::{Playlist, Track};
use crate::auth::get_stored_token;

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
