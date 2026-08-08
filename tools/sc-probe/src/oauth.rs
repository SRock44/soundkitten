//! Official OAuth 2.1 + PKCE verification flow (Phase 0, step 5/6).
//! Loopback-redirect flow suitable for a native/CLI app per RFC 8252.

use anyhow::{bail, Context, Result};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use rand::Rng;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::io::Write;
use std::time::Duration;

const AUTHORIZE_URL: &str = "https://secure.soundcloud.com/authorize";
const TOKEN_URL: &str = "https://secure.soundcloud.com/oauth/token";
const OFFICIAL_API: &str = "https://api.soundcloud.com";

fn random_url_safe(len: usize) -> String {
    let bytes: Vec<u8> = (0..len).map(|_| rand::thread_rng().r#gen()).collect();
    URL_SAFE_NO_PAD.encode(bytes)
}

fn pkce_challenge(verifier: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());
    URL_SAFE_NO_PAD.encode(hasher.finalize())
}

struct AuthCode {
    code: String,
}

fn wait_for_redirect(port: u16, expected_state: &str) -> Result<AuthCode> {
    let server = tiny_http::Server::http(format!("127.0.0.1:{port}"))
        .map_err(|e| anyhow::anyhow!("failed to bind loopback listener on {port}: {e}"))?;

    server.server_addr();
    let request = server
        .recv_timeout(Duration::from_secs(180))
        .context("waiting for OAuth redirect")?
        .context("timed out waiting for OAuth redirect (180s) — did you complete the login in the browser?")?;

    let url = format!("http://127.0.0.1:{port}{}", request.url());
    let parsed = url::Url::parse(&url)?;
    let params: std::collections::HashMap<_, _> = parsed.query_pairs().into_owned().collect();

    let response_html = if params.contains_key("code") {
        "<html><body>Login successful — you can close this tab and return to sc-probe.</body></html>"
    } else {
        "<html><body>Login failed or was denied — check sc-probe output.</body></html>"
    };
    let response = tiny_http::Response::from_string(response_html)
        .with_header("Content-Type: text/html".parse::<tiny_http::Header>().unwrap());
    let _ = request.respond(response);

    if let Some(err) = params.get("error") {
        bail!("SoundCloud returned an OAuth error: {err} ({})", params.get("error_description").cloned().unwrap_or_default());
    }

    let state = params.get("state").context("redirect missing state param")?;
    if state != expected_state {
        bail!("state mismatch — possible CSRF or stale redirect (expected {expected_state}, got {state})");
    }

    let code = params.get("code").context("redirect missing code param")?.clone();
    Ok(AuthCode { code })
}

pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
}

pub async fn run_oauth_flow(client_id: &str, client_secret: &str, port: u16) -> Result<Tokens> {
    let verifier = random_url_safe(64);
    let challenge = pkce_challenge(&verifier);
    let state = random_url_safe(16);
    let redirect_uri = format!("http://127.0.0.1:{port}/callback");

    let auth_url = url::Url::parse_with_params(
        AUTHORIZE_URL,
        &[
            ("client_id", client_id),
            ("redirect_uri", &redirect_uri),
            ("response_type", "code"),
            ("code_challenge", &challenge),
            ("code_challenge_method", "S256"),
            ("state", &state),
        ],
    )?;

    println!("Opening browser for SoundCloud login...");
    println!("(If it doesn't open automatically, visit: {auth_url})");
    let _ = open::that(auth_url.as_str());

    print!("Waiting for redirect to {redirect_uri} ... ");
    std::io::stdout().flush().ok();
    let auth_code = wait_for_redirect(port, &state)?;
    println!("received code.");

    let client = reqwest::Client::new();
    let resp = client
        .post(TOKEN_URL)
        .form(&[
            ("grant_type", "authorization_code"),
            ("client_id", client_id),
            ("client_secret", client_secret),
            ("redirect_uri", &redirect_uri),
            ("code", &auth_code.code),
            ("code_verifier", &verifier),
        ])
        .send()
        .await?;

    let status = resp.status();
    let body: Value = resp.json().await.context("parsing token response")?;
    if !status.is_success() {
        bail!("token exchange returned {status}: {body}");
    }

    let access_token = body
        .get("access_token")
        .and_then(|v| v.as_str())
        .context("token response missing access_token")?
        .to_string();
    let refresh_token = body
        .get("refresh_token")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let expires_in = body.get("expires_in").and_then(|v| v.as_u64()).unwrap_or(0);

    Ok(Tokens { access_token, refresh_token, expires_in })
}

pub async fn refresh_tokens(client_id: &str, client_secret: &str, refresh_token: &str) -> Result<Tokens> {
    let client = reqwest::Client::new();
    let resp = client
        .post(TOKEN_URL)
        .form(&[
            ("grant_type", "refresh_token"),
            ("client_id", client_id),
            ("client_secret", client_secret),
            ("refresh_token", refresh_token),
        ])
        .send()
        .await?;
    let status = resp.status();
    let body: Value = resp.json().await.context("parsing refresh response")?;
    if !status.is_success() {
        bail!("refresh_token exchange returned {status}: {body}");
    }
    Ok(Tokens {
        access_token: body.get("access_token").and_then(|v| v.as_str()).unwrap_or_default().to_string(),
        refresh_token: body.get("refresh_token").and_then(|v| v.as_str()).unwrap_or_default().to_string(),
        expires_in: body.get("expires_in").and_then(|v| v.as_u64()).unwrap_or(0),
    })
}

pub async fn authed_official_get(client: &reqwest::Client, path: &str, access_token: &str) -> Result<Value> {
    let resp = client
        .get(format!("{OFFICIAL_API}{path}"))
        .header("Authorization", format!("Bearer {access_token}"))
        .send()
        .await?;
    let status = resp.status();
    let text = resp.text().await?;
    let body: Value = serde_json::from_str(&text).unwrap_or(Value::String(text.clone()));
    if !status.is_success() {
        bail!("{path} returned {status}: {body}");
    }
    Ok(body)
}
