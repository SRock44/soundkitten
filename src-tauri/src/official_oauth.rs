//! Official OAuth 2.1 + PKCE, second and separate from the primary
//! cookie-based login in auth.rs. Used only to enable real like/unlike and
//! follow/unfollow, both of which are DataDome-blocked on the unofficial
//! API (see src-tauri/src/soundcloud/api.rs). Live-verified against a real
//! account and a real deployed proxy before this was written, see
//! docs/oauth-migration.md and tools/sc-probe/src/oauth.rs.
//!
//! The token exchange never touches SoundCloud's secret directly: this app
//! only ever calls PROXY_BASE (a small server the developer controls, see
//! services/oauth-proxy/), which is the only place client_secret exists.

use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tauri::AppHandle;

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use rand::Rng;
use sha2::{Digest, Sha256};

// Public identifier, not a credential -- safe to embed. Only client_secret,
// which never appears anywhere in this app, needs protecting.
const CLIENT_ID: &str = "MlbYb5ThUhbWHcXlZZ7fgMDcoGKOgyIs";
const AUTHORIZE_URL: &str = "https://secure.soundcloud.com/authorize";
const OFFICIAL_API: &str = "https://api.soundcloud.com";
const PROXY_BASE: &str = "https://auth.soundkitten.org";
// Must match the redirect_uri registered with the SoundCloud app.
const REDIRECT_PORT: u16 = 8765;

const KEYRING_SERVICE: &str = "com.soundkitten.app.official";
const KEYRING_USER: &str = "official_oauth_tokens";

// Abuse deterrent for the proxy, not a real secret (see
// services/oauth-proxy/, comment there explains why this one is fine to
// ship in the binary, unlike SC_CLIENT_SECRET, which never appears here).
const PROXY_ACCESS_KEY: &str = "737719365c6ab79840724fe0375f70bb61f3556f0fbb980e";

fn http_client() -> reqwest::Client {
    reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) soundcloud-desktop/0.1")
        .build()
        .expect("failed to build reqwest client")
}

#[derive(Serialize, Deserialize, Clone)]
struct StoredTokens {
    access_token: String,
    refresh_token: String,
    /// unix seconds
    expires_at: u64,
}

fn keyring_entry() -> anyhow::Result<keyring::Entry> {
    Ok(keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER)?)
}

fn load_tokens() -> Option<StoredTokens> {
    let raw = keyring_entry().ok()?.get_password().ok()?;
    serde_json::from_str(&raw).ok()
}

fn store_tokens(tokens: &StoredTokens) -> anyhow::Result<()> {
    let raw = serde_json::to_string(tokens)?;
    keyring_entry()?.set_password(&raw)?;
    Ok(())
}

fn clear_tokens() {
    if let Ok(entry) = keyring_entry() {
        let _ = entry.delete_credential();
    }
}

fn now_unix() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs()
}

#[tauri::command]
pub fn is_official_connected() -> bool {
    load_tokens().is_some()
}

#[tauri::command]
pub fn disconnect_official_login() {
    clear_tokens();
}

// --- PKCE + loopback redirect ---

fn random_url_safe(len: usize) -> String {
    let bytes: Vec<u8> = (0..len).map(|_| rand::thread_rng().gen()).collect();
    URL_SAFE_NO_PAD.encode(bytes)
}

fn pkce_challenge(verifier: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());
    URL_SAFE_NO_PAD.encode(hasher.finalize())
}

fn redirect_uri() -> String {
    format!("http://127.0.0.1:{REDIRECT_PORT}/callback")
}

fn build_authorize_url(challenge: &str, state: &str, redirect_uri: &str) -> anyhow::Result<url::Url> {
    Ok(url::Url::parse_with_params(
        AUTHORIZE_URL,
        &[
            ("client_id", CLIENT_ID),
            ("redirect_uri", redirect_uri),
            ("response_type", "code"),
            ("code_challenge", challenge),
            ("code_challenge_method", "S256"),
            ("state", state),
        ],
    )?)
}

/// Blocks the current (spawned) thread waiting for the one OAuth redirect.
/// Runs off the async runtime via spawn_blocking, mirroring the pattern
/// tools/sc-probe/src/oauth.rs already proved works for this exact flow.
fn wait_for_redirect(port: u16, expected_state: &str) -> anyhow::Result<String> {
    let server = tiny_http::Server::http(format!("127.0.0.1:{port}"))
        .map_err(|e| anyhow::anyhow!("failed to bind loopback listener on {port}: {e}"))?;

    let request = server
        .recv_timeout(Duration::from_secs(180))?
        .ok_or_else(|| anyhow::anyhow!("timed out waiting for OAuth redirect (180s)"))?;

    let url = format!("http://127.0.0.1:{port}{}", request.url());
    let parsed = url::Url::parse(&url)?;
    let params: std::collections::HashMap<_, _> = parsed.query_pairs().into_owned().collect();

    let response_html = if params.contains_key("code") {
        "<html><body>Connected -- you can close this tab and return to SoundKitten.</body></html>"
    } else {
        "<html><body>Login failed or was denied -- check SoundKitten and try again.</body></html>"
    };
    let response = tiny_http::Response::from_string(response_html)
        .with_header("Content-Type: text/html".parse::<tiny_http::Header>().unwrap());
    let _ = request.respond(response);

    if let Some(err) = params.get("error") {
        anyhow::bail!("SoundCloud returned an OAuth error: {err}");
    }
    let state = params.get("state").ok_or_else(|| anyhow::anyhow!("redirect missing state param"))?;
    if state != expected_state {
        anyhow::bail!("state mismatch, possible CSRF or stale redirect");
    }
    params.get("code").cloned().ok_or_else(|| anyhow::anyhow!("redirect missing code param"))
}

async fn exchange_code_via_proxy(code: &str, verifier: &str, redirect_uri: &str) -> anyhow::Result<StoredTokens> {
    let client = http_client();
    let resp = client
        .post(format!("{PROXY_BASE}/token/exchange"))
        .header("x-soundkitten-key", PROXY_ACCESS_KEY)
        .json(&serde_json::json!({
            "code": code,
            "code_verifier": verifier,
            "redirect_uri": redirect_uri,
        }))
        .send()
        .await?;
    parse_token_response(resp).await
}

async fn refresh_via_proxy(refresh_token: &str) -> anyhow::Result<StoredTokens> {
    let client = http_client();
    let resp = client
        .post(format!("{PROXY_BASE}/token/refresh"))
        .header("x-soundkitten-key", PROXY_ACCESS_KEY)
        .json(&serde_json::json!({ "refresh_token": refresh_token }))
        .send()
        .await?;
    parse_token_response(resp).await
}

async fn parse_token_response(resp: reqwest::Response) -> anyhow::Result<StoredTokens> {
    let status = resp.status();
    let body: serde_json::Value = resp.json().await?;
    if !status.is_success() {
        anyhow::bail!("token proxy returned {status}: {body}");
    }
    let access_token = body.get("access_token").and_then(|v| v.as_str()).ok_or_else(|| anyhow::anyhow!("response missing access_token"))?.to_string();
    let refresh_token = body.get("refresh_token").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let expires_in = body.get("expires_in").and_then(|v| v.as_u64()).unwrap_or(3600);
    Ok(StoredTokens {
        access_token,
        refresh_token,
        expires_at: now_unix() + expires_in,
    })
}

/// Returns a currently-valid access token, refreshing via the proxy first if
/// the stored one is expired or about to expire.
async fn get_valid_access_token() -> Result<String, String> {
    let Some(tokens) = load_tokens() else {
        return Err("not connected".into());
    };
    if tokens.expires_at > now_unix() + 60 {
        return Ok(tokens.access_token);
    }
    if tokens.refresh_token.is_empty() {
        return Err("official session expired, please reconnect".into());
    }
    let refreshed = refresh_via_proxy(&tokens.refresh_token).await.map_err(|e| e.to_string())?;
    store_tokens(&refreshed).map_err(|e| e.to_string())?;
    Ok(refreshed.access_token)
}

/// The official OAuth connect flow, in the system browser per RFC 8252's
/// recommendation for native-app OAuth. Called two ways: automatically,
/// right after primary login succeeds (see auth.rs's start_login), so it
/// reads as one continuous onboarding moment; and lazily, from the
/// frontend's officialAuth store, if the user declined or closed the
/// browser tab the first time and now wants to like or follow something.
#[tauri::command]
pub async fn start_official_login(app: AppHandle) -> Result<(), String> {
    let verifier = random_url_safe(64);
    let challenge = pkce_challenge(&verifier);
    let state = random_url_safe(16);
    let redirect_uri = redirect_uri();
    let auth_url = build_authorize_url(&challenge, &state, &redirect_uri).map_err(|e| e.to_string())?;

    // Bind the listener BEFORE opening the browser so the redirect always
    // has somewhere to land.
    let state_for_wait = state.clone();
    let wait_handle = tauri::async_runtime::spawn_blocking(move || wait_for_redirect(REDIRECT_PORT, &state_for_wait));

    use tauri_plugin_opener::OpenerExt;
    app.opener().open_url(auth_url.as_str(), None::<&str>).map_err(|e| e.to_string())?;

    let code = wait_handle
        .await
        .map_err(|e| format!("login task panicked: {e}"))?
        .map_err(|e| e.to_string())?;

    let tokens = exchange_code_via_proxy(&code, &verifier, &redirect_uri).await.map_err(|e| e.to_string())?;
    store_tokens(&tokens).map_err(|e| e.to_string())
}

// --- Writes that only work via official OAuth (DataDome-blocked otherwise) ---

async fn official_write(method: reqwest::Method, path: &str) -> Result<(), String> {
    let access_token = get_valid_access_token().await?;
    let client = http_client();
    let resp = client
        .request(method, format!("{OFFICIAL_API}{path}"))
        .header("Authorization", format!("Bearer {access_token}"))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let status = resp.status();
    if status.is_success() {
        return Ok(());
    }
    let body = resp.text().await.unwrap_or_default();
    Err(format!("{status}: {body}"))
}

#[tauri::command]
pub async fn sc_like_track_v2(track_id: i64) -> Result<(), String> {
    official_write(reqwest::Method::POST, &format!("/likes/tracks/{track_id}")).await
}

#[tauri::command]
pub async fn sc_unlike_track_v2(track_id: i64) -> Result<(), String> {
    official_write(reqwest::Method::DELETE, &format!("/likes/tracks/{track_id}")).await
}

#[tauri::command]
pub async fn sc_like_playlist_v2(playlist_id: i64) -> Result<(), String> {
    official_write(reqwest::Method::POST, &format!("/likes/playlists/{playlist_id}")).await
}

#[tauri::command]
pub async fn sc_unlike_playlist_v2(playlist_id: i64) -> Result<(), String> {
    official_write(reqwest::Method::DELETE, &format!("/likes/playlists/{playlist_id}")).await
}

#[tauri::command]
pub async fn sc_follow_user_v2(user_id: i64) -> Result<(), String> {
    official_write(reqwest::Method::PUT, &format!("/me/followings/{user_id}")).await
}

#[tauri::command]
pub async fn sc_unfollow_user_v2(user_id: i64) -> Result<(), String> {
    official_write(reqwest::Method::DELETE, &format!("/me/followings/{user_id}")).await
}
