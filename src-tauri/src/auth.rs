//! Primary login: one embedded-webview screen, pointed at SoundCloud's
//! official /authorize page rather than the plain login page. You can't
//! authorize a third-party app without a SoundCloud session, so this still
//! shows the normal login form first if the user isn't already signed in,
//! sets the same oauth_token cookie the unofficial API needs (extracted the
//! same way it always was), and then -- if the user also hits Allow on the
//! consent screen -- redirects with an authorization code, which we
//! exchange for official OAuth tokens too (see official_oauth.rs). One
//! login screen, both credential sets. If the user closes the window or
//! declines consent, primary login still succeeds on the cookie alone;
//! they just get official_oauth's lazy connect prompt later on first like
//! or follow, same as if they'd never seen this screen.

use serde::Serialize;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter, Manager, WebviewUrl, WebviewWindowBuilder};

use crate::official_oauth;

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

#[tauri::command]
pub async fn start_login(app: AppHandle) -> Result<(), String> {
    if app.get_webview_window(LOGIN_WINDOW_LABEL).is_some() {
        return Ok(()); // login already in progress
    }

    let verifier = official_oauth::random_url_safe(64);
    let challenge = official_oauth::pkce_challenge(&verifier);
    let state = official_oauth::random_url_safe(16);
    let redirect_uri = official_oauth::redirect_uri();
    let auth_url = official_oauth::build_authorize_url(&challenge, &state, &redirect_uri).map_err(|e| e.to_string())?;

    // Bind the loopback listener before the window can possibly navigate
    // there, same reasoning as official_oauth::start_official_login.
    let state_for_wait = state.clone();
    let wait_handle = tauri::async_runtime::spawn_blocking(move || official_oauth::wait_for_redirect(official_oauth::REDIRECT_PORT, &state_for_wait));

    let _window = WebviewWindowBuilder::new(&app, LOGIN_WINDOW_LABEL, WebviewUrl::External(auth_url.as_str().parse().map_err(|e| format!("{e}"))?))
        .title("Log in to SoundCloud")
        .inner_size(480.0, 760.0)
        .build()
        .map_err(|e| e.to_string())?;

    let app_for_poll = app.clone();
    tauri::async_runtime::spawn(async move {
        let deadline = Instant::now() + Duration::from_secs(300);
        let sc_url: url::Url = "https://soundcloud.com".parse().unwrap();

        // Phase 1, required: wait for the oauth_token cookie. Playback and
        // browsing depend on this; the official side (phase 2) is a bonus.
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
                    if let Err(e) = store_token(&token) {
                        emit_result(&app_for_poll, false, Some(format!("failed to store token: {e}")));
                        close_login_window(&app_for_poll);
                        return;
                    }
                    break;
                }
            }

            tokio::time::sleep(Duration::from_millis(750)).await;
        }

        // Phase 2, best-effort: the window is still open, likely on the
        // consent screen now. Give the user a chance to hit Allow so we can
        // pick up the official credentials too, without holding primary
        // login's success hostage to it -- if they close the window or
        // deny, that's fine, they'll see the lazy connect prompt later.
        let window_closed = async {
            loop {
                if app_for_poll.get_webview_window(LOGIN_WINDOW_LABEL).is_none() {
                    return;
                }
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        };
        tokio::select! {
            result = wait_handle => {
                if let Ok(Ok(code)) = result {
                    let _ = official_oauth::complete_login(&code, &verifier, &redirect_uri).await;
                }
            }
            _ = window_closed => {}
        }

        close_login_window(&app_for_poll);
        emit_result(&app_for_poll, true, None);
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
