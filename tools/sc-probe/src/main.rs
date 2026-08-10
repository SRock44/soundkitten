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

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max])
    }
}

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
    /// Verify official-API playlist create/update/delete, and dump raw
    /// /stream JSON to confirm the repost-origin field shape (v0.2.1
    /// playlist CRUD + feed-origin work)
    PlaylistFeedSpike {
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

    let me = check!(
        "authed GET /me (official api.soundcloud.com)",
        oauth::authed_official_get(&client, "/me", &tokens.access_token).await
    );
    let _my_id = me.as_ref().and_then(|v| v.get("id")).and_then(|v| v.as_i64());

    let likes = check!(
        "authed GET /me/likes/tracks (official)",
        oauth::authed_official_get(&client, "/me/likes/tracks", &tokens.access_token).await
    );
    let liked_track_id = likes
        .as_ref()
        .and_then(|v| v.as_array())
        .and_then(|a| a.first())
        .and_then(|t| t.get("id"))
        .and_then(|v| v.as_i64());

    check!(
        "authed GET /me/playlists (official)",
        oauth::authed_official_get(&client, "/me/playlists", &tokens.access_token).await
    );

    check!(
        "authed GET /me/tracks (official, user's own uploads)",
        oauth::authed_official_get(&client, "/me/tracks", &tokens.access_token).await
    );

    // --- Phase 0 open questions from docs/oauth-migration.md ---
    println!("\n--- Phase 0: migration open questions ---\n");

    // Q3/Q4/Q5: does the official API expose the surface we actually need?
    check!("GET /tracks?q=... (search)", async {
        let (status, body) = oauth::official_get_raw(&client, "/tracks?q=lofi&limit=3", &tokens.access_token).await?;
        if !status.is_success() {
            bail!("{status}: {body}");
        }
        Ok(body)
    }.await);

    let track_check = check!("GET /tracks/{id} (streaming/transcoding data)", async {
        let id = liked_track_id.context("no liked track available to test with (like a track first)")?;
        let (status, body) = oauth::official_get_raw(&client, &format!("/tracks/{id}"), &tokens.access_token).await?;
        if !status.is_success() {
            bail!("{status}: {body}");
        }
        Ok(body)
    }.await);
    if let Some(track) = &track_check {
        let has_stream_url = track.get("stream_url").is_some();
        let has_media = track.get("media").is_some();
        let streamable = track.get("streamable").and_then(|v| v.as_bool()).unwrap_or(false);
        println!("    stream_url field present: {has_stream_url}, media field present: {has_media}, streamable: {streamable}");
        if has_stream_url {
            let stream_url = track.get("stream_url").and_then(|v| v.as_str()).unwrap_or_default();
            let resolved = client
                .get(stream_url)
                .header("Authorization", format!("Bearer {}", tokens.access_token))
                .send()
                .await;
            match resolved {
                Ok(r) => println!("    following stream_url -> HTTP {} (redirects followed: {})", r.status(), r.url()),
                Err(e) => println!("    following stream_url -> request failed: {e}"),
            }
        }
    }

    if let Some(id) = liked_track_id {
        check!("GET /tracks/{id}/comments (official)", async {
            let (status, body) = oauth::official_get_raw(&client, &format!("/tracks/{id}/comments"), &tokens.access_token).await?;
            if !status.is_success() {
                bail!("{status}: {body}");
            }
            Ok(body)
        }.await);
    } else {
        total += 1;
        println!("[{total}] GET /tracks/{{id}}/comments (official) ... SKIPPED (no liked track id available)");
    }

    check!("GET /me/followers (official)", oauth::authed_official_get(&client, "/me/followers", &tokens.access_token).await);
    let followings = check!("GET /me/followings (official)", oauth::authed_official_get(&client, "/me/followings", &tokens.access_token).await);
    let followed_user_id = followings
        .as_ref()
        .and_then(|v| v.as_array())
        .and_then(|a| a.first())
        .and_then(|u| u.get("id"))
        .and_then(|v| v.as_i64());

    check!("GET /me/activities (official feed equivalent)", oauth::authed_official_get(&client, "/me/activities", &tokens.access_token).await);

    // mixed-selections is a v2-web-app-specific endpoint, expected to 404 on the official host -- checking anyway rather than assuming
    total += 1;
    print!("[{total}] GET /mixed-selections (official, expected to NOT exist) ... ");
    std::io::stdout().flush().ok();
    match oauth::official_get_raw(&client, "/mixed-selections", &tokens.access_token).await {
        Ok((status, _)) if status == 404 => {
            println!("CONFIRMED ABSENT (404, as expected)");
            passed += 1;
        }
        Ok((status, body)) => println!("UNEXPECTED: got {status} instead of 404: {body}"),
        Err(e) => println!("request failed: {e:#}"),
    }

    // Q1: does official likes/follows bypass DataDome? Idempotent tests only
    // (re-liking an already-liked track / re-following an already-followed
    // user), so nothing changes even on success.
    if let Some(id) = liked_track_id {
        println!("\n    Trying several candidate paths for the official like-write endpoint (the guessed /me/favorites/{{id}} 405'd as \"unknown route\" last run, so the real path may differ or may not exist at all):");
        for (method, path) in [
            (reqwest::Method::PUT, format!("/me/favorites/{id}")),
            (reqwest::Method::POST, format!("/me/favorites/{id}")),
            (reqwest::Method::PUT, format!("/tracks/{id}/favoriters")),
            (reqwest::Method::PUT, format!("/likes/tracks/{id}")),
            (reqwest::Method::PUT, format!("/me/likes/tracks/{id}")),
            (reqwest::Method::PUT, format!("/tracks/{id}/likers")),
            (reqwest::Method::POST, format!("/tracks/{id}/likes")),
            (reqwest::Method::PUT, format!("/me/favorites/tracks/{id}")),
            (reqwest::Method::PUT, format!("/likes/{id}")),
        ] {
            match oauth::official_write(&client, method.clone(), &path, &tokens.access_token).await {
                Ok((status, body)) => println!("      {method} {path} -> {status}: {}", truncate(&body.to_string(), 120)),
                Err(e) => println!("      {method} {path} -> request failed: {e}"),
            }
        }
        // POST with the track id in the body instead of the path, in case the
        // real shape is "collection create" rather than a path-parameterized resource.
        for (body_desc, form) in [
            ("track_id form field", vec![("track_id", id.to_string())]),
            ("id form field", vec![("id", id.to_string())]),
        ] {
            let resp = client
                .post("https://api.soundcloud.com/me/likes/tracks")
                .header("Authorization", format!("Bearer {}", tokens.access_token))
                .form(&form)
                .send()
                .await;
            match resp {
                Ok(r) => println!("      POST /me/likes/tracks ({body_desc}) -> {}", r.status()),
                Err(e) => println!("      POST /me/likes/tracks ({body_desc}) -> request failed: {e}"),
            }
        }
        // OPTIONS on the known-working read path, to see what methods the
        // server itself declares as valid there (Allow header), rather than
        // guessing paths blind.
        let opts = client
            .request(reqwest::Method::OPTIONS, "https://api.soundcloud.com/me/likes/tracks")
            .header("Authorization", format!("Bearer {}", tokens.access_token))
            .send()
            .await;
        match opts {
            Ok(r) => {
                let allow = r.headers().get("Allow").and_then(|v| v.to_str().ok()).unwrap_or("<no Allow header>").to_string();
                println!("      OPTIONS /me/likes/tracks -> {} , Allow: {allow}", r.status());
            }
            Err(e) => println!("      OPTIONS /me/likes/tracks -> request failed: {e}"),
        }
        total += 1;
        passed += 1; // informational sweep, not a pass/fail check
        println!("    (see above for results, this sweep doesn't count as pass/fail)\n");

        // Targeted follow-up: POST/DELETE /likes/tracks/{track_urn} specifically.
        // Not covered above, that sweep only tried PUT on /likes/tracks/{id}
        // with a bare numeric id, never POST/DELETE, and never the URN form
        // (soundcloud:tracks:<id>) SoundCloud uses as the canonical resource
        // identifier elsewhere in their API.
        let urn = track_check
            .as_ref()
            .and_then(|t| t.get("urn"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("soundcloud:tracks:{id}"));
        println!("    Targeted check: POST/DELETE /likes/tracks/{{track_urn}} (urn = {urn}, track has urn field: {}):", track_check.as_ref().and_then(|t| t.get("urn")).is_some());

        let urn_encoded = urn.replace(':', "%3A");
        for path in [
            format!("/likes/tracks/{id}"),
            format!("/likes/tracks/{urn}"),
            format!("/likes/tracks/{urn_encoded}"),
        ] {
            match oauth::official_write(&client, reqwest::Method::POST, &path, &tokens.access_token).await {
                Ok((status, body)) => {
                    println!("      POST {path} -> {status}: {}", truncate(&body.to_string(), 150));
                    if status.is_success() {
                        // Real endpoint. Confirm DELETE (unlike) works, then
                        // immediately restore the like so account state ends
                        // up unchanged from before this run.
                        let del = oauth::official_write(&client, reqwest::Method::DELETE, &path, &tokens.access_token).await;
                        match &del {
                            Ok((s, b)) => println!("      DELETE {path} -> {s}: {}", truncate(&b.to_string(), 150)),
                            Err(e) => println!("      DELETE {path} -> request failed: {e}"),
                        }
                        let restore = oauth::official_write(&client, reqwest::Method::POST, &path, &tokens.access_token).await;
                        println!("      restore POST {path} -> {:?}", restore.map(|(s, _)| s));
                    }
                }
                Err(e) => println!("      POST {path} -> request failed: {e}"),
            }
        }
        println!();
    } else {
        total += 1;
        println!("[{total}] like-endpoint sweep ... SKIPPED (no existing like to target)");
    }

    // Playlist likes: same documented shape (POST/DELETE /likes/playlists/{playlist_urn}),
    // confirming it actually behaves the same as the now-confirmed track-like endpoint
    // rather than assuming it does just because the docs list it identically.
    let playlists = check!(
        "authed GET /me/playlists (for playlist-like check)",
        oauth::authed_official_get(&client, "/me/playlists", &tokens.access_token).await
    );
    if let Some(playlist_id) = playlists.as_ref().and_then(|v| v.as_array()).and_then(|a| a.first()).and_then(|p| p.get("id")).and_then(|v| v.as_i64()) {
        let path = format!("/likes/playlists/{playlist_id}");
        println!("    Targeted check: POST/DELETE {path} (playlist likes):");
        match oauth::official_write(&client, reqwest::Method::POST, &path, &tokens.access_token).await {
            Ok((status, body)) => {
                println!("      POST {path} -> {status}: {}", truncate(&body.to_string(), 150));
                if status.is_success() {
                    let del = oauth::official_write(&client, reqwest::Method::DELETE, &path, &tokens.access_token).await;
                    match &del {
                        Ok((s, b)) => println!("      DELETE {path} -> {s}: {}", truncate(&b.to_string(), 150)),
                        Err(e) => println!("      DELETE {path} -> request failed: {e}"),
                    }
                    let restore = oauth::official_write(&client, reqwest::Method::POST, &path, &tokens.access_token).await;
                    println!("      restore POST {path} -> {:?}", restore.map(|(s, _)| s));
                }
            }
            Err(e) => println!("      POST {path} -> request failed: {e}"),
        }
        println!();
    } else {
        println!("    (no playlist available to test playlist-like endpoint)\n");
    }

    // Known-public account used only to test whether a follow WRITE endpoint
    // exists at all. If any candidate succeeds, immediately unfollow again to
    // avoid leaving a new, unwanted follow relationship behind.
    let follow_test_target = followed_user_id.unwrap_or(991556812);
    println!("    Trying several candidate paths for the official follow-write endpoint (target user id {follow_test_target}{}):", if followed_user_id.is_some() { ", an existing following" } else { ", NOT currently followed, will clean up if any candidate succeeds" });
    for (method, path) in [
        (reqwest::Method::PUT, format!("/me/followings/{follow_test_target}")),
        (reqwest::Method::POST, format!("/me/followings/{follow_test_target}")),
        (reqwest::Method::PUT, format!("/users/{follow_test_target}/followers")),
    ] {
        match oauth::official_write(&client, method.clone(), &path, &tokens.access_token).await {
            Ok((status, body)) => {
                println!("      {method} {path} -> {status}: {}", truncate(&body.to_string(), 120));
                if status.is_success() && followed_user_id.is_none() {
                    let cleanup = oauth::official_write(&client, reqwest::Method::DELETE, &path, &tokens.access_token).await;
                    println!("      cleanup DELETE {path} -> {:?}", cleanup.map(|(s, _)| s));
                }
            }
            Err(e) => println!("      {method} {path} -> request failed: {e}"),
        }
    }
    total += 1;
    passed += 1;
    println!("    (see above for results, this sweep doesn't count as pass/fail)\n");

    // Q6: can the official access_token be used against the UNOFFICIAL api-v2 host?
    check!("official access_token against unofficial api-v2.soundcloud.com", async {
        let client_id_v2 = scrape_client_id(&client).await.context("scraping unofficial client_id for this check")?;
        let (status, body) = oauth::try_official_token_on_unofficial_api(&client, &tokens.access_token, &client_id_v2).await?;
        if !status.is_success() {
            bail!("{status}: {body}");
        }
        Ok(body)
    }.await);

    // Q2: is client_secret actually enforced, or would PKCE alone have been enough?
    if !tokens.refresh_token.is_empty() {
        total += 1;
        print!("[{total}] refresh_token exchange WITHOUT client_secret (is it actually enforced?) ... ");
        std::io::stdout().flush().ok();
        let resp = client
            .post("https://secure.soundcloud.com/oauth/token")
            .form(&[("grant_type", "refresh_token"), ("client_id", &client_id), ("refresh_token", &tokens.refresh_token)])
            .send()
            .await;
        match resp {
            Ok(r) if r.status().is_success() => {
                println!("ACCEPTED without a secret -- client_secret is NOT strictly enforced here");
                passed += 1;
            }
            Ok(r) => {
                println!("REJECTED ({}) -- client_secret appears to be required", r.status());
                passed += 1; // this is a valid, informative answer either way
            }
            Err(e) => println!("request failed: {e:#}"),
        }
    }

    if !tokens.refresh_token.is_empty() {
        let refreshed = check!(
            "refresh_token exchange for a new access_token (normal, with secret)",
            oauth::refresh_tokens(&client_id, &client_secret, &tokens.refresh_token).await
        );
        if let Some(r) = refreshed {
            println!("    new access_token acquired (expires_in={}s)", r.expires_in);
        }
    } else {
        total += 1;
        println!("[{total}] refresh_token exchange ... SKIPPED (no refresh_token returned)");
    }

    // End-to-end test of the deployed oauth-proxy (services/oauth-proxy, now
    // running as a Node service on node1 behind auth.soundkitten.org): does
    // the SAME refresh_token, sent through the public proxy instead of
    // directly to SoundCloud, come back with a valid access_token? This is
    // the one thing the proxy's own negative tests (405/401/400 checks)
    // couldn't prove, since none of them reach the code path that actually
    // reads SC_CLIENT_ID/SC_CLIENT_SECRET from the container's environment
    // and forwards to SoundCloud. Never prints the refresh_token or the
    // resulting access_token, only whether one came back.
    if !tokens.refresh_token.is_empty() {
        if let Ok(proxy_key) = std::env::var("PROXY_ACCESS_KEY") {
            total += 1;
            print!("[{total}] refresh_token exchange via deployed oauth-proxy (auth.soundkitten.org) ... ");
            std::io::stdout().flush().ok();
            let resp = client
                .post("https://auth.soundkitten.org/token/refresh")
                .header("x-soundkitten-key", &proxy_key)
                .json(&serde_json::json!({ "refresh_token": tokens.refresh_token }))
                .send()
                .await;
            match resp {
                Ok(r) => {
                    let status = r.status();
                    let body: Value = r.json().await.unwrap_or(Value::Null);
                    let has_access_token = body.get("access_token").and_then(|v| v.as_str()).is_some();
                    if status.is_success() && has_access_token {
                        println!("PASS (proxy forwarded to SoundCloud and returned a valid access_token)");
                        passed += 1;
                    } else {
                        println!("FAIL: {status}, access_token present: {has_access_token}");
                    }
                }
                Err(e) => println!("FAIL: request failed: {e:#}"),
            }
        } else {
            total += 1;
            println!("[{total}] refresh_token exchange via deployed oauth-proxy ... SKIPPED (PROXY_ACCESS_KEY not set)");
        }
    }

    println!("\n=== {passed}/{total} official-API checks passed ===");
    println!("\nNOTE: download-endpoint availability on the official API still needs manual verification");
    println!("against a track you have download rights to, not yet automated here.");
    if passed < total {
        std::process::exit(1);
    }
    Ok(())
}

/// Verifies playlist create/update/delete against the official API (never
/// tried in this codebase before -- likes/follows writes were confirmed via
/// the sweep above, but playlist mutation is a genuine unknown), and dumps
/// raw /stream JSON so the real repost-origin field shape can be read off
/// directly rather than assumed. Cleans up everything it creates.
async fn run_playlist_feed_spike(port: u16) -> Result<()> {
    let _ = dotenvy::from_filename(".env.local");
    let client_id = std::env::var("SC_CLIENT_ID").context("SC_CLIENT_ID not set (put it in .env.local)")?;
    let client_secret = std::env::var("SC_CLIENT_SECRET").context("SC_CLIENT_SECRET not set (put it in .env.local)")?;

    println!("=== sc-probe: playlist CRUD + feed-origin verification ===\n");

    let tokens = oauth::run_oauth_flow(&client_id, &client_secret, port).await?;
    println!("access_token acquired.\n");
    let client = reqwest::Client::new();
    let at = &tokens.access_token;

    // --- Playlist CRUD ---
    println!("--- Playlist create/update/delete ---\n");

    let create_body = serde_json::json!({ "playlist": { "title": "sc-probe test (safe to delete)", "sharing": "private", "tracks": [] } });
    let (status, body) = oauth::official_write_json(&client, reqwest::Method::POST, "/playlists", at, &create_body).await?;
    println!("POST /playlists (empty tracks) -> {status}: {}", truncate(&body.to_string(), 400));
    let Some(playlist_id) = body.get("id").and_then(|v| v.as_i64()) else {
        bail!("playlist creation didn't return an id -- can't continue the spike. Response above is the thing to read.");
    };
    println!("    created playlist id = {playlist_id}\n");

    // Need a real track id to test the tracks array with -- reuse a liked track.
    let likes = oauth::authed_official_get(&client, "/me/likes/tracks", at).await.ok();
    let track_id = likes.as_ref().and_then(|v| v.as_array()).and_then(|a| a.first()).and_then(|t| t.get("id")).and_then(|v| v.as_i64());

    if let Some(track_id) = track_id {
        let update_body = serde_json::json!({ "playlist": { "title": "sc-probe test (safe to delete)", "tracks": [{ "id": track_id }] } });
        let (status, body) = oauth::official_write_json(&client, reqwest::Method::PUT, &format!("/playlists/{playlist_id}"), at, &update_body).await?;
        println!("PUT /playlists/{playlist_id} (add track {track_id}) -> {status}: {}", truncate(&body.to_string(), 400));
        let returned_tracks = body.pointer("/tracks").and_then(|t| t.as_array()).map(|a| a.len());
        println!("    tracks array length in response: {returned_tracks:?}\n");

        let clear_body = serde_json::json!({ "playlist": { "title": "sc-probe test (safe to delete)", "tracks": [] } });
        let (status, body) = oauth::official_write_json(&client, reqwest::Method::PUT, &format!("/playlists/{playlist_id}"), at, &clear_body).await?;
        println!("PUT /playlists/{playlist_id} (clear tracks, confirms whole-array-replace) -> {status}: {}", truncate(&body.to_string(), 300));
        println!();
    } else {
        println!("(no liked track available to test the tracks array with -- like a track first for full coverage)\n");
    }

    let (status, body) = oauth::official_write(&client, reqwest::Method::DELETE, &format!("/playlists/{playlist_id}"), at).await?;
    println!("DELETE /playlists/{playlist_id} (cleanup) -> {status}: {}\n", truncate(&body.to_string(), 200));

    // --- Feed origin shape ---
    println!("--- /stream item shapes (for FeedItem/FeedEntry field names) ---\n");
    match oauth::authed_official_get(&client, "/me/activities?limit=20", at).await {
        Ok(body) => {
            let collection = body.get("collection").and_then(|c| c.as_array()).cloned().unwrap_or_default();
            println!("collection length: {}", collection.len());
            let repost = collection.iter().find(|i| i.get("type").and_then(|t| t.as_str()).is_some_and(|t| t.contains("repost")));
            let plain = collection.iter().find(|i| i.get("type").and_then(|t| t.as_str()).is_some_and(|t| !t.contains("repost")));
            match repost {
                Some(item) => println!("\nfirst REPOST item (full JSON, read the reposter/timestamp field names off this):\n{}", serde_json::to_string_pretty(item).unwrap_or_default()),
                None => println!("\nno repost item found in the first 20 activities -- widen the search or try again later"),
            }
            match plain {
                Some(item) => println!("\nfirst PLAIN (non-repost) item (full JSON, for comparison):\n{}", serde_json::to_string_pretty(item).unwrap_or_default()),
                None => println!("\nno plain item found in the first 20 activities"),
            }
        }
        Err(e) => println!("GET /me/activities failed: {e:#}"),
    }

    println!("\n=== spike complete -- read the raw JSON above to write FeedEntry/playlist-write field shapes ===");
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

    match &args.command {
        Some(Command::Oauth { port }) => return run_oauth_checks(*port).await,
        Some(Command::PlaylistFeedSpike { port }) => return run_playlist_feed_spike(*port).await,
        None => {}
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
