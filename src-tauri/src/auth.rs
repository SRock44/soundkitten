//! Login flow: embedded webview -> extract oauth_token cookie -> OS keychain.
//! Falls back to a manual-token-paste command when cookie extraction fails.

use serde::Serialize;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter, Manager, WebviewUrl, WebviewWindowBuilder};

const KEYRING_SERVICE: &str = "com.soundkitten.app";
const KEYRING_USER: &str = "oauth_token";
const LOGIN_WINDOW_LABEL: &str = "sc-login";

fn keyring_entry() -> anyhow::Result<keyring::Entry> {
    Ok(keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER)?)
}

pub fn get_stored_token() -> Option<String> {
    keyring_entry().ok()?.get_password().ok()
}

fn store_token(token: &str) -> anyhow::Result<()> {
    keyring_entry()?.set_password(token)?;
    Ok(())
}

#[derive(Serialize, Clone)]
struct AuthEvent {
    ok: bool,
    error: Option<String>,
}

#[tauri::command]
pub fn is_logged_in() -> bool {
    get_stored_token().is_some()
}

#[tauri::command]
pub fn logout() -> Result<(), String> {
    if let Ok(entry) = keyring_entry() {
        // Absence of a stored credential is not an error here.
        let _ = entry.delete_credential();
    }
    Ok(())
}

/// Manual fallback: user pastes their oauth_token cookie value directly.
#[tauri::command]
pub fn set_manual_token(token: String) -> Result<(), String> {
    let token = token.trim();
    if token.is_empty() {
        return Err("token was empty".into());
    }
    store_token(token).map_err(|e| e.to_string())
}

/// Opens a login window pointed at soundcloud.com, polls the webview's cookie
/// jar until an oauth_token cookie appears (i.e. the user finished logging in),
/// stores it, closes the window, and emits `auth:result` to the frontend.
#[tauri::command]
pub async fn start_login(app: AppHandle) -> Result<(), String> {
    if app.get_webview_window(LOGIN_WINDOW_LABEL).is_some() {
        return Ok(()); // login already in progress
    }

    let _window = WebviewWindowBuilder::new(
        &app,
        LOGIN_WINDOW_LABEL,
        WebviewUrl::External("https://soundcloud.com/login".parse().map_err(|e| format!("{e}"))?),
    )
    .title("Log in to SoundCloud")
    .inner_size(480.0, 720.0)
    .build()
    .map_err(|e| e.to_string())?;

    let app_for_poll = app.clone();
    tauri::async_runtime::spawn(async move {
        let deadline = Instant::now() + Duration::from_secs(300);
        let sc_url: url::Url = "https://soundcloud.com".parse().unwrap();

        loop {
            if Instant::now() >= deadline {
                emit_result(&app_for_poll, false, Some("login timed out after 5 minutes".into()));
                close_login_window(&app_for_poll);
                return;
            }

            let Some(win) = app_for_poll.get_webview_window(LOGIN_WINDOW_LABEL) else {
                // window was closed by the user before completing login
                emit_result(&app_for_poll, false, Some("login window closed before completing login".into()));
                return;
            };

            // cookies() can deadlock if called synchronously on some platforms;
            // we're already off the main thread here via async_runtime::spawn.
            let cookies_result = win.cookies_for_url(sc_url.clone());
            if let Ok(cookies) = cookies_result {
                if let Some(token_cookie) = cookies.iter().find(|c| c.name() == "oauth_token") {
                    let token = token_cookie.value().to_string();
                    match store_token(&token) {
                        Ok(()) => emit_result(&app_for_poll, true, None),
                        Err(e) => emit_result(&app_for_poll, false, Some(format!("failed to store token: {e}"))),
                    }
                    close_login_window(&app_for_poll);
                    return;
                }
            }

            tokio::time::sleep(Duration::from_millis(750)).await;
        }
    });

    Ok(())
}

fn close_login_window(app: &AppHandle) {
    if let Some(win) = app.get_webview_window(LOGIN_WINDOW_LABEL) {
        let _ = win.close();
    }
}

fn emit_result(app: &AppHandle, ok: bool, error: Option<String>) {
    let _ = app.emit("auth:result", AuthEvent { ok, error });
}

/// Verifies the stored token actually works by hitting an authenticated endpoint.
#[tauri::command]
pub async fn verify_auth() -> Result<bool, String> {
    let Some(token) = get_stored_token() else {
        return Ok(false);
    };
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) soundcloud-desktop/0.1")
        .build()
        .map_err(|e| e.to_string())?;
    match crate::soundcloud::authed_get::<serde_json::Value>(&client, "/me", &[], Some(&token)).await {
        Ok(_) => Ok(true),
        Err(e) => Err(e.to_string()),
    }
}
