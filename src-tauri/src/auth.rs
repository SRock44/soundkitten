//! Primary login: embedded webview -> extract oauth_token cookie -> OS
//! keychain. This is the exact mechanism that's worked since 0.1.0.
//!
//! An earlier version of this file tried pointing the same webview at
//! SoundCloud's official /authorize page instead, hoping to capture the
//! official OAuth authorization code from the same screen (one login for
//! everything). Tried twice, live, and failed both times: the oauth_token
//! cookie this relies on never appeared, regardless of which cookie origin
//! was checked, for reasons that weren't diagnosable without direct
//! webview devtools access. Reverted rather than keep guessing against a
//! mechanism that's now failed twice under real testing.
//!
//! Instead: primary login stays exactly as it always was (proven), and on
//! success, automatically kicks off official_oauth::start_official_login
//! (an embedded popup window, see that module) right after, so it reads as
//! one continuous onboarding moment instead of a separately-discovered
//! "connect" action later, without touching primary login's own mechanism.
//! Its outcome is reported via `official_auth:result` so the frontend
//! knows immediately whether it's actually connected, rather than only
//! finding out the next time it happens to check.

use serde::Serialize;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter, Manager, WebviewUrl, WebviewWindowBuilder};

use crate::official_oauth;

const KEYRING_SERVICE: &str = "com.soundkitten.app";
const KEYRING_USER: &str = "oauth_token";
const LOGIN_WINDOW_LABEL: &str = "sc-login";

fn keyring_entry() -> anyhow::Result<keyring::Entry> {
    Ok(keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER)?)
}

/// In-process cache of the stored token, populated on first read. Without
/// a stable Apple Developer signature, macOS ties Keychain access approval
/// to the app's exact code signature, which changes on every build
/// (including every auto-update), so it can't recognize "already approved"
/// across versions and re-prompts. get_stored_token() is called from
/// nearly every backend command, so reading straight from the OS keychain
/// every time turned that into dozens of prompts in a single startup burst
/// (reported live: "like 70 times" right after an update). Reading once
/// per process and caching the result cuts that down to essentially one
/// prompt per session, the actual per-signature re-approval is still real
/// and can't be avoided without notarization, but it no longer compounds.
fn token_cache() -> &'static Mutex<Option<String>> {
    static CACHE: OnceLock<Mutex<Option<String>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(None))
}

pub fn get_stored_token() -> Option<String> {
    if let Some(token) = token_cache().lock().unwrap().as_ref() {
        return Some(token.clone());
    }
    let token = keyring_entry().ok()?.get_password().ok()?;
    *token_cache().lock().unwrap() = Some(token.clone());
    Some(token)
}

fn store_token(token: &str) -> anyhow::Result<()> {
    keyring_entry()?.set_password(token)?;
    *token_cache().lock().unwrap() = Some(token.to_string());
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
    *token_cache().lock().unwrap() = None;
    Ok(())
}

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
                        Err(e) => {
                            emit_result(&app_for_poll, false, Some(format!("failed to store token: {e}")));
                            close_login_window(&app_for_poll);
                            return;
                        }
                    }
                    close_login_window(&app_for_poll);

                    // Primary login just succeeded. Automatically continue
                    // into the official OAuth connect popup right away,
                    // best-effort, so the user encounters it as part of one
                    // onboarding moment instead of separately discovering
                    // it later on first like or follow. Doesn't affect
                    // primary login, which already reported success above
                    // either way. The frontend's officialAuth store listens
                    // for this event to update its connected state right
                    // away, instead of only learning about it on next
                    // launch -- without this, a successful onboarding
                    // connect would still prompt again on the very first
                    // like or follow.
                    let app_for_official = app_for_poll.clone();
                    tauri::async_runtime::spawn(async move {
                        let ok = official_oauth::start_official_login(app_for_official.clone()).await.is_ok();
                        let _ = app_for_official.emit("official_auth:result", ok);
                    });
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
