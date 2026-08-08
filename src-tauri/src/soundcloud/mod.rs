//! Unofficial SoundCloud v2 API client: client_id scraping + authed reads.
//! Auth technique: browser oauth_token cookie, same as validated in tools/sc-probe.

pub mod api;
pub mod commands;
pub mod models;

use regex::Regex;
use serde::de::DeserializeOwned;
use std::sync::OnceLock;
use std::sync::RwLock;

const API_V2: &str = "https://api-v2.soundcloud.com";
const WEB_APP: &str = "https://soundcloud.com";

static CACHED_CLIENT_ID: OnceLock<RwLock<Option<String>>> = OnceLock::new();

fn cache() -> &'static RwLock<Option<String>> {
    CACHED_CLIENT_ID.get_or_init(|| RwLock::new(None))
}

/// Scrapes a working client_id from soundcloud.com's public JS bundles.
/// This is the same unofficial technique validated in tools/sc-probe.
pub async fn fetch_client_id(client: &reqwest::Client) -> anyhow::Result<String> {
    let html = client.get(WEB_APP).send().await?.text().await?;

    let script_re = Regex::new(r#"src="(https://a-v2\.sndcdn\.com/assets/[^"]+\.js)""#)?;
    let script_urls: Vec<String> = script_re.captures_iter(&html).map(|c| c[1].to_string()).collect();
    if script_urls.is_empty() {
        anyhow::bail!("no sndcdn asset script tags found — SoundCloud's bundle layout may have changed");
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
    anyhow::bail!("scanned {} bundle(s) but found no client_id pattern", script_urls.len())
}

/// Returns a cached client_id, fetching (and caching) a fresh one if needed.
pub async fn get_client_id(client: &reqwest::Client) -> anyhow::Result<String> {
    if let Some(id) = cache().read().unwrap().clone() {
        return Ok(id);
    }
    let id = fetch_client_id(client).await?;
    *cache().write().unwrap() = Some(id.clone());
    Ok(id)
}

/// GET an absolute URL (e.g. a `next_href` pagination cursor) with client_id
/// (+ optional OAuth auth) appended, for following pagination cursors that
/// SoundCloud returns as full URLs rather than relative paths.
pub async fn get_full_url<T: DeserializeOwned>(
    client: &reqwest::Client,
    url: &str,
    oauth_token: Option<&str>,
) -> anyhow::Result<T> {
    let client_id = get_client_id(client).await?;
    let mut req = client.get(url).query(&[("client_id", client_id.as_str())]);
    if let Some(token) = oauth_token {
        req = req.header("Authorization", format!("OAuth {token}"));
    }
    let resp = req.send().await?;
    let status = resp.status();
    let text = resp.text().await?;
    if !status.is_success() {
        anyhow::bail!("{url} returned {status}: {}", if text.is_empty() { "<empty body>" } else { &text });
    }
    let body: T = serde_json::from_str(&text)
        .map_err(|e| anyhow::anyhow!("{url} returned {status} but body didn't match the expected shape: {e}"))?;
    Ok(body)
}

/// GET /resolve?url=... -- used for turning a soundcloud.com URL into a track/playlist object.
pub async fn resolve_raw<T: DeserializeOwned>(
    client: &reqwest::Client,
    url: &str,
    oauth_token: Option<&str>,
) -> anyhow::Result<T> {
    authed_get(client, "/resolve", &[("url", url)], oauth_token).await
}

/// PUT an api-v2 path with client_id + OAuth auth and no body -- used for
/// like/repost-style "create this relationship" actions.
pub async fn authed_put(client: &reqwest::Client, path: &str, oauth_token: &str) -> anyhow::Result<()> {
    authed_mutate(client, reqwest::Method::PUT, path, oauth_token).await
}

/// DELETE an api-v2 path with client_id + OAuth auth -- used for
/// unlike/unrepost-style "remove this relationship" actions.
pub async fn authed_delete(client: &reqwest::Client, path: &str, oauth_token: &str) -> anyhow::Result<()> {
    authed_mutate(client, reqwest::Method::DELETE, path, oauth_token).await
}

async fn authed_mutate(client: &reqwest::Client, method: reqwest::Method, path: &str, oauth_token: &str) -> anyhow::Result<()> {
    let client_id = get_client_id(client).await?;
    let resp = client
        .request(method, format!("{API_V2}{path}"))
        .query(&[("client_id", client_id.as_str())])
        .header("Authorization", format!("OAuth {oauth_token}"))
        .send()
        .await?;
    let status = resp.status();
    if !status.is_success() {
        let text = resp.text().await.unwrap_or_default();
        anyhow::bail!("{path} returned {status}: {}", if text.is_empty() { "<empty body>" } else { &text });
    }
    Ok(())
}

/// POST an api-v2 path with a JSON body, client_id + OAuth auth.
pub async fn authed_post<T: DeserializeOwned>(
    client: &reqwest::Client,
    path: &str,
    body: &serde_json::Value,
    oauth_token: &str,
) -> anyhow::Result<T> {
    let client_id = get_client_id(client).await?;
    let resp = client
        .post(format!("{API_V2}{path}"))
        .query(&[("client_id", client_id.as_str())])
        .header("Authorization", format!("OAuth {oauth_token}"))
        .json(body)
        .send()
        .await?;
    let status = resp.status();
    let text = resp.text().await?;
    if !status.is_success() {
        anyhow::bail!("{path} returned {status}: {}", if text.is_empty() { "<empty body>" } else { &text });
    }
    let parsed: T = serde_json::from_str(&text)
        .map_err(|e| anyhow::anyhow!("{path} returned {status} but body didn't match the expected shape: {e}"))?;
    Ok(parsed)
}

/// Manual override, e.g. from a Settings screen, used when scraping breaks.
#[tauri::command]
pub fn set_client_id_override(id: String) {
    *cache().write().unwrap() = Some(id);
}

/// GET an api-v2 path with client_id (+ optional OAuth cookie auth), refreshing
/// client_id once on 401, and deserializing into `T` with tolerant field handling
/// (unknown/missing fields don't panic -- see soundcloud::models).
pub async fn authed_get<T: DeserializeOwned>(
    client: &reqwest::Client,
    path: &str,
    query_extra: &[(&str, &str)],
    oauth_token: Option<&str>,
) -> anyhow::Result<T> {
    let mut client_id = get_client_id(client).await?;
    for attempt in 0..2 {
        let mut query: Vec<(&str, &str)> = vec![("client_id", client_id.as_str())];
        query.extend_from_slice(query_extra);

        let mut req = client.get(format!("{API_V2}{path}")).query(&query);
        if let Some(token) = oauth_token {
            req = req.header("Authorization", format!("OAuth {token}"));
        }

        let resp = req.send().await?;
        let status = resp.status();
        let text = resp.text().await?;

        if status == reqwest::StatusCode::UNAUTHORIZED && attempt == 0 {
            // client_id may be stale; force a fresh scrape and retry once.
            client_id = fetch_client_id(client).await?;
            *cache().write().unwrap() = Some(client_id.clone());
            continue;
        }
        if !status.is_success() {
            anyhow::bail!("{path} returned {status}: {}", if text.is_empty() { "<empty body>" } else { &text });
        }
        let body: T = serde_json::from_str(&text)
            .map_err(|e| anyhow::anyhow!("{path} returned {status} but body didn't match the expected shape: {e}"))?;
        return Ok(body);
    }
    unreachable!()
}
