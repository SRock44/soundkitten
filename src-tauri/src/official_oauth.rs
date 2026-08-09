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
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

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
const OFFICIAL_LOGIN_WINDOW_LABEL: &str = "sc-official-login";

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

fn bind_loopback_listener(port: u16) -> anyhow::Result<tiny_http::Server> {
    tiny_http::Server::http(format!("127.0.0.1:{port}")).map_err(|e| anyhow::anyhow!("failed to bind loopback listener on {port}: {e}"))
}

/// Blocks the current (spawned) thread waiting for the one OAuth redirect,
/// polling in short increments rather than one long recv_timeout so it can
/// notice `cancelled` promptly. This matters: if the caller gives up on
/// this listener (e.g. the popup window was closed before a redirect
/// arrived) without a way to interrupt an in-progress wait, the OS thread
/// spawn_blocking runs this on keeps the port bound regardless -- dropping
/// a spawn_blocking JoinHandle does NOT cancel the underlying thread, only
/// detaches it. A leaked listener here means the NEXT connect attempt
/// either fails to bind port 8765 at all, or worse, its real redirect gets
/// delivered to this stale, orphaned listener instead (which then rejects
/// it for a state mismatch), while the new attempt just hangs until its
/// own timeout. Both were reproduced live before this fix.
fn wait_for_redirect(server: &tiny_http::Server, expected_state: &str, cancelled: &AtomicBool) -> anyhow::Result<String> {
    let deadline = Instant::now() + Duration::from_secs(180);
    loop {
        if cancelled.load(Ordering::Relaxed) {
            anyhow::bail!("cancelled");
        }
        if Instant::now() >= deadline {
            anyhow::bail!("timed out waiting for OAuth redirect (180s)");
        }

        let request = match server.recv_timeout(Duration::from_millis(400))? {
            Some(r) => r,
            None => continue, // this 400ms slice elapsed with nothing incoming, recheck cancelled/deadline
        };

        let url = format!("http://127.0.0.1{}", request.url());
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
        return params.get("code").cloned().ok_or_else(|| anyhow::anyhow!("redirect missing code param"));
    }
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

/// The official OAuth connect flow, shown as an embedded popup window
/// rather than the system browser. It's a separate WebviewWindow in the
/// same app as auth.rs's primary login window, sharing the same WebView2
/// profile/cookies, so if the user already has an active SoundCloud
/// session from primary login, /authorize should recognize it and go
/// straight to the consent screen instead of asking them to log in again.
/// The window is closed automatically as soon as an answer comes back
/// (code, denial, or the user closing it themselves), no waiting on the
/// user to close it. Called two ways: automatically, right after primary
/// login succeeds (see auth.rs's start_login), so it reads as one
/// continuous onboarding moment; and lazily, from the frontend's
/// officialAuth store, if the user declined the first time and now wants
/// to like or follow something.
///
/// This still relies only on the PKCE + loopback-redirect mechanism
/// that's independently proven (tools/sc-probe/src/oauth.rs, and this same
/// function's own system-browser predecessor), not on reading any cookie
/// out of the popup, which is the part that broke in an earlier attempt to
/// merge this into the primary login webview itself.
#[tauri::command]
pub async fn start_official_login(app: AppHandle) -> Result<(), String> {
    if app.get_webview_window(OFFICIAL_LOGIN_WINDOW_LABEL).is_some() {
        return Ok(()); // already in progress
    }

    let verifier = random_url_safe(64);
    let challenge = pkce_challenge(&verifier);
    let state = random_url_safe(16);
    let redirect_uri = redirect_uri();
    let auth_url = build_authorize_url(&challenge, &state, &redirect_uri).map_err(|e| e.to_string())?;

    // Bind the listener here (fast, non-blocking) rather than inside the
    // spawned thread, so this Arc can also be handed to the cancellation
    // path below -- see wait_for_redirect's doc comment for why that
    // matters.
    let server = Arc::new(bind_loopback_listener(REDIRECT_PORT).map_err(|e| e.to_string())?);
    let cancelled = Arc::new(AtomicBool::new(false));

    let server_for_wait = server.clone();
    let cancelled_for_wait = cancelled.clone();
    let state_for_wait = state.clone();
    let wait_handle = tauri::async_runtime::spawn_blocking(move || wait_for_redirect(&server_for_wait, &state_for_wait, &cancelled_for_wait));

    let _window = WebviewWindowBuilder::new(&app, OFFICIAL_LOGIN_WINDOW_LABEL, WebviewUrl::External(auth_url.as_str().parse().map_err(|e| format!("{e}"))?))
        .title("Connect to SoundCloud")
        .inner_size(480.0, 720.0)
        .build()
        .map_err(|e| e.to_string())?;

    // Race the redirect against the user closing the popup themselves, so
    // declining doesn't leave the command hanging until the loopback
    // listener's own 180s timeout.
    let app_for_watch = app.clone();
    let window_closed = async move {
        loop {
            if app_for_watch.get_webview_window(OFFICIAL_LOGIN_WINDOW_LABEL).is_none() {
                return;
            }
            tokio::time::sleep(Duration::from_millis(400)).await;
        }
    };
    let redirect_result = tokio::select! {
        r = wait_handle => Some(r),
        _ = window_closed => {
            // Tell the still-running blocking thread to give up within its
            // next ~400ms poll instead of holding port 8765 for up to 180s.
            cancelled.store(true, Ordering::Relaxed);
            None
        }
    };

    // Auto-close: the user shouldn't have to close this themselves once
    // we have an answer either way.
    if let Some(win) = app.get_webview_window(OFFICIAL_LOGIN_WINDOW_LABEL) {
        let _ = win.close();
    }

    let code = redirect_result
        .ok_or_else(|| "login window closed before completing".to_string())?
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
