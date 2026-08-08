//! Throwaway verification probe for Phase 0.
//! Confirms the unofficial SoundCloud API surface this project depends on
//! actually works, before any real app code is written against it.
//!
//! Usage:
//!   sc-probe --track-url <public track url> --download-track-url <a track you have Go+ download rights to>
//!   (oauth token is read from SC_OAUTH_TOKEN env var, or pasted interactively if unset)

mod oauth;

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use regex::Regex;
use serde_json::Value;
use std::io::Write;

const API_V2: &str = "https://api-v2.soundcloud.com";
const WEB_APP: &str = "https://soundcloud.com";

#[derive(Parser)]
struct Args {
    /// Public track URL to test /resolve and streaming against
    #[arg(long, default_value = "https://soundcloud.com/skrillex/first-of-the-year-equinox")]
    track_url: String,

    /// Search query to test /search/tracks
    #[arg(long, default_value = "lofi")]
    query: String,

    /// A track URL you have Go+ download rights to (optional, skips check 6 if omitted)
    #[arg(long)]
    download_track_url: Option<String>,

    /// Skip authenticated checks entirely (no interactive prompt)
    #[arg(long)]
    no_auth: bool,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Verify the official OAuth 2.1 + PKCE flow end-to-end
    Oauth {
        /// Local loopback port for the redirect_uri (must be allowed by the registered app, if it restricts redirect URIs)
        #[arg(long, default_value_t = 8765)]
        port: u16,
    },
}

async fn run_oauth_checks(port: u16) -> Result<()> {
    let _ = dotenvy::from_filename(".env.local");
    let client_id = std::env::var("SC_CLIENT_ID").context("SC_CLIENT_ID not set (put it in .env.local)")?;
    let client_secret = std::env::var("SC_CLIENT_SECRET").context("SC_CLIENT_SECRET not set (put it in .env.local)")?;

    println!("=== sc-probe: official OAuth 2.1 + PKCE verification ===\n");

    let mut passed = 0u8;
    let mut total = 0u8;
    macro_rules! check {
        ($n:expr, $body:expr) => {{
            total += 1;
            print!("[{}] {} ... ", total, $n);
            std::io::stdout().flush().ok();
            match $body {
                Ok(v) => {
                    println!("PASS");
                    passed += 1;
                    Some(v)
                }
                Err(e) => {
                    println!("FAIL: {e:#}");
                    None
                }
            }
        }};
    }

    let tokens = check!(
        "run authorization code + PKCE flow, exchange code for tokens",
        oauth::run_oauth_flow(&client_id, &client_secret, port).await
    );
    let Some(tokens) = tokens else {
        println!("\nCannot continue without tokens. {passed}/{total} passed.");
        std::process::exit(1);
    };
    println!(
        "    access_token acquired (expires_in={}s), refresh_token {}",
        tokens.expires_in,
        if tokens.refresh_token.is_empty() { "MISSING" } else { "present" }
    );

    let client = reqwest::Client::new();

    check!(
        "authed GET /me (official api.soundcloud.com)",
        oauth::authed_official_get(&client, "/me", &tokens.access_token).await
    );

    check!(
        "authed GET /me/likes/tracks (official)",
        oauth::authed_official_get(&client, "/me/likes/tracks", &tokens.access_token).await
    );

    check!(
        "authed GET /me/playlists (official)",
        oauth::authed_official_get(&client, "/me/playlists", &tokens.access_token).await
    );

    check!(
        "authed GET /me/tracks (official, user's own uploads)",
        oauth::authed_official_get(&client, "/me/tracks", &tokens.access_token).await
    );

    if !tokens.refresh_token.is_empty() {
        let refreshed = check!(
            "refresh_token exchange for a new access_token",
            oauth::refresh_tokens(&client_id, &client_secret, &tokens.refresh_token).await
        );
        if let Some(r) = refreshed {
            println!("    new access_token acquired (expires_in={}s)", r.expires_in);
        }
    } else {
        total += 1;
        println!("[{total}] refresh_token exchange ... SKIPPED (no refresh_token returned)");
    }

    println!("\n=== {passed}/{total} official-API checks passed ===");
    println!("\nNOTE: download-endpoint availability on the official API still needs manual verification");
    println!("against a track you have download rights to — not yet automated here.");
    if passed < total {
        std::process::exit(1);
    }
    Ok(())
}

fn get_oauth_token(no_auth: bool) -> Result<Option<String>> {
    if no_auth {
        return Ok(None);
    }
    if let Ok(t) = std::env::var("SC_OAUTH_TOKEN") {
        if !t.trim().is_empty() {
            return Ok(Some(t.trim().to_string()));
        }
    }
    print!("Paste your soundcloud.com `oauth_token` cookie value (blank to skip auth checks): ");
    std::io::stdout().flush()?;
    let token = rpassword::read_password().unwrap_or_default();
    let token = token.trim().to_string();
    Ok(if token.is_empty() { None } else { Some(token) })
}

async fn scrape_client_id(client: &reqwest::Client) -> Result<String> {
    let html = client
        .get(WEB_APP)
        .send()
        .await
        .context("fetching soundcloud.com homepage")?
        .text()
        .await?;

    let script_re = Regex::new(r#"src="(https://a-v2\.sndcdn\.com/assets/[^"]+\.js)""#)?;
    let script_urls: Vec<String> = script_re
        .captures_iter(&html)
        .map(|c| c[1].to_string())
        .collect();

    if script_urls.is_empty() {
        bail!("no sndcdn asset script tags found in homepage HTML — SoundCloud's bundle layout may have changed");
    }

    let id_re = Regex::new(r#"client_id["']?\s*[:=]\s*["']([a-zA-Z0-9]{32})["']"#)?;

    for url in &script_urls {
        let js = match client.get(url).send().await {
            Ok(r) => r.text().await.unwrap_or_default(),
            Err(_) => continue,
        };
        if let Some(cap) = id_re.captures(&js) {
            return Ok(cap[1].to_string());
        }
    }

    bail!("scanned {} bundle(s) but found no client_id pattern", script_urls.len())
}

async fn check_resolve(client: &reqwest::Client, client_id: &str, track_url: &str) -> Result<Value> {
    let resp = client
        .get(format!("{API_V2}/resolve"))
        .query(&[("url", track_url), ("client_id", client_id)])
        .send()
        .await?;
    let status = resp.status();
    let body: Value = resp.json().await.context("parsing /resolve response as JSON")?;
    if !status.is_success() {
        bail!("/resolve returned {status}: {body}");
    }
    if body.get("kind").is_none() {
        bail!("/resolve response missing expected `kind` field: {body}");
    }
    Ok(body)
}

async fn check_search(client: &reqwest::Client, client_id: &str, query: &str) -> Result<()> {
    let resp = client
        .get(format!("{API_V2}/search/tracks"))
        .query(&[("q", query), ("client_id", client_id), ("limit", "5")])
        .send()
        .await?;
    let status = resp.status();
    let body: Value = resp.json().await.context("parsing /search/tracks response")?;
    if !status.is_success() {
        bail!("/search/tracks returned {status}: {body}");
    }
    let collection = body
        .get("collection")
        .and_then(|c| c.as_array())
        .context("/search/tracks response missing `collection` array")?;
    if collection.is_empty() {
        bail!("/search/tracks returned zero results for query {query:?}");
    }
    Ok(())
}

async fn authed_get(
    client: &reqwest::Client,
    path: &str,
    client_id: &str,
    oauth_token: &str,
) -> Result<Value> {
    let resp = client
        .get(format!("{API_V2}{path}"))
        .query(&[("client_id", client_id), ("limit", "5")])
        .header("Authorization", format!("OAuth {oauth_token}"))
        .send()
        .await?;
    let status = resp.status();
    let text = resp.text().await.context("reading authed response body")?;
    if !status.is_success() {
        bail!("{path} returned {status}: {}", if text.is_empty() { "<empty body>" } else { &text });
    }
    let body: Value = serde_json::from_str(&text)
        .with_context(|| format!("{path} returned {status} but body wasn't JSON: {text:.200}"))?;
    Ok(body)
}

async fn check_stream(client: &reqwest::Client, client_id: &str, track: &Value) -> Result<()> {
    let transcodings = track
        .pointer("/media/transcodings")
        .and_then(|t| t.as_array())
        .context("track missing media.transcodings")?;

    let progressive = transcodings
        .iter()
        .find(|t| t.pointer("/format/protocol").and_then(|p| p.as_str()) == Some("progressive"))
        .or_else(|| transcodings.first())
        .context("no transcodings available on track")?;

    let stream_api_url = progressive
        .get("url")
        .and_then(|u| u.as_str())
        .context("transcoding missing url")?;

    let resp = client
        .get(stream_api_url)
        .query(&[("client_id", client_id)])
        .send()
        .await?;
    let status = resp.status();
    let body: Value = resp.json().await.context("parsing stream resolution response")?;
    if !status.is_success() {
        bail!("stream resolution returned {status}: {body}");
    }
    let stream_url = body
        .get("url")
        .and_then(|u| u.as_str())
        .context("stream resolution missing `url`")?;

    let mut audio_resp = client.get(stream_url).send().await?;
    if !audio_resp.status().is_success() {
        bail!("fetching resolved stream URL returned {}", audio_resp.status());
    }
    let mut total = 0usize;
    while let Some(chunk) = audio_resp.chunk().await? {
        total += chunk.len();
        if total >= 8192 {
            break;
        }
    }
    if total == 0 {
        bail!("stream URL returned zero bytes");
    }
    Ok(())
}

async fn check_download(
    client: &reqwest::Client,
    client_id: &str,
    oauth_token: &str,
    track: &Value,
) -> Result<()> {
    let track_id = track.get("id").context("track missing id")?;
    let downloadable = track.get("downloadable").and_then(|v| v.as_bool()).unwrap_or(false);
    let has_downloads_left = track.get("has_downloads_left").and_then(|v| v.as_bool());
    println!(
        "\n    (track {track_id}: downloadable={downloadable}, has_downloads_left={has_downloads_left:?})"
    );
    let resp = client
        .get(format!("{API_V2}/tracks/{track_id}/download"))
        .query(&[("client_id", client_id)])
        .header("Authorization", format!("OAuth {oauth_token}"))
        .send()
        .await?;
    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        bail!(
            "/tracks/{track_id}/download returned {status}: {} (downloadable flag on track was {downloadable}) — if downloadable=false, pick a different track that actually has downloads enabled by its uploader",
            if body.is_empty() { "<empty body>".to_string() } else { body.chars().take(200).collect() }
        );
    }
    let body: Value = resp.json().await.context("parsing download response")?;
    let redirect_url = body
        .get("redirectUri")
        .and_then(|u| u.as_str())
        .context("download response missing redirectUri")?;

    let mut audio_resp = client.get(redirect_url).send().await?;
    if !audio_resp.status().is_success() {
        bail!("fetching download redirectUri returned {}", audio_resp.status());
    }
    let mut total = 0usize;
    while let Some(chunk) = audio_resp.chunk().await? {
        total += chunk.len();
        if total >= 8192 {
            break;
        }
    }
    if total == 0 {
        bail!("download URL returned zero bytes");
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(Command::Oauth { port }) = &args.command {
        return run_oauth_checks(*port).await;
    }

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) sc-probe/0.1")
        .build()?;

    let mut passed = 0u8;
    let mut total = 0u8;
    macro_rules! check {
        ($n:expr, $body:expr) => {{
            total += 1;
            print!("[{}] {} ... ", total, $n);
            std::io::stdout().flush().ok();
            match $body {
                Ok(v) => {
                    println!("PASS");
                    passed += 1;
                    Some(v)
                }
                Err(e) => {
                    println!("FAIL: {e:#}");
                    None
                }
            }
        }};
    }

    println!("=== sc-probe: Phase 0 live API verification ===\n");

    let client_id = check!("scrape client_id from soundcloud.com bundles", scrape_client_id(&client).await);
    let Some(client_id) = client_id else {
        println!("\nCannot continue without a client_id. {passed}/{total} passed.");
        std::process::exit(1);
    };
    println!("    client_id = {client_id}");

    let resolved = check!(
        "resolve public track URL via /resolve",
        check_resolve(&client, &client_id, &args.track_url).await
    );

    check!(
        "search /search/tracks",
        check_search(&client, &client_id, &args.query).await
    );

    if let Some(track) = &resolved {
        check!("resolve stream URL and fetch audio bytes", check_stream(&client, &client_id, track).await);
    } else {
        total += 1;
        println!("[{total}] resolve stream URL and fetch audio bytes ... SKIPPED (resolve failed)");
    }

    let oauth_token = get_oauth_token(args.no_auth)?;
    if let Some(token) = &oauth_token {
        let likes = check!(
            "authed GET /me/track_likes",
            authed_get(&client, "/me/track_likes", &client_id, token).await
        );
        check!(
            "authed GET /me/playlists",
            authed_get(&client, "/me/playlists", &client_id, token).await
        );
        let _ = likes;

        if let Some(dl_url) = &args.download_track_url {
            let dl_resolved = check_resolve(&client, &client_id, dl_url).await;
            match dl_resolved {
                Ok(track) => {
                    check!(
                        "Go+ original download endpoint returns real audio",
                        check_download(&client, &client_id, token, &track).await
                    );
                }
                Err(e) => {
                    total += 1;
                    println!("[{total}] Go+ original download endpoint ... FAIL: could not resolve --download-track-url: {e:#}");
                }
            }
        } else {
            total += 1;
            println!("[{total}] Go+ original download endpoint ... SKIPPED (pass --download-track-url to test)");
        }
    } else {
        total += 3;
        println!("[{}] authed checks ... SKIPPED (no oauth token provided, x3)", total - 2);
    }

    println!("\n=== {passed}/{total} checks passed ===");
    if passed < total {
        std::process::exit(1);
    }
    Ok(())
}
