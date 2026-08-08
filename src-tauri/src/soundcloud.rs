//! Minimal unofficial SoundCloud API client: client_id scraping + authed reads.
//! Auth technique: browser oauth_token cookie, same as validated in tools/sc-probe.

use regex::Regex;
use serde_json::Value;
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

/// Manual override, e.g. from a Settings screen, used when scraping breaks.
#[tauri::command]
pub fn set_client_id_override(id: String) {
    *cache().write().unwrap() = Some(id);
}

/// GET an api-v2 path with client_id + OAuth cookie auth, refreshing client_id once on 401.
pub async fn authed_get(client: &reqwest::Client, path: &str, oauth_token: &str) -> anyhow::Result<Value> {
    let mut client_id = get_client_id(client).await?;
    for attempt in 0..2 {
        let resp = client
            .get(format!("{API_V2}{path}"))
            .query(&[("client_id", client_id.as_str()), ("limit", "20")])
            .header("Authorization", format!("OAuth {oauth_token}"))
            .send()
            .await?;
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
        let body: Value = serde_json::from_str(&text)
            .map_err(|e| anyhow::anyhow!("{path} returned {status} but body wasn't valid JSON: {e}"))?;
        return Ok(body);
    }
    unreachable!()
}
