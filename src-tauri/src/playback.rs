//! `sc-stream://` custom protocol: proxies authenticated SoundCloud audio
//! bytes to the frontend's <audio> element, since <audio src> can't attach
//! the Authorization header or client_id itself. Streaming only -- bytes are
//! piped straight to the webview, never written to a user-accessible file.

use base64::{engine::general_purpose::STANDARD, Engine};
use tauri::http::{Request, Response, StatusCode};
use tauri::{UriSchemeContext, UriSchemeResponder};

/// WebView2 (Windows) has a known quirk where non-2xx responses from custom
/// URI scheme handlers surface to the page as a bare network failure
/// ("Failed to fetch") instead of a normal HTTP error the frontend can read
/// -- so real errors are never actually a non-2xx status here. Instead we
/// always answer 200 and put the real status + a base64'd message (headers
/// must be ASCII, and error text may contain arbitrary characters) in
/// custom headers; the frontend checks for X-Sc-Error explicitly rather
/// than trusting `response.ok`.
fn error_response(logical_status: StatusCode, message: String) -> Response<Vec<u8>> {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain")
        .header("X-Sc-Status", logical_status.as_str())
        .header("X-Sc-Error", STANDARD.encode(&message))
        // Without this, the browser's CORS check silently blocks the response
        // from ever reaching JS (fetch() just sees "Failed to fetch"), since
        // this is a cross-origin request from the app's own origin to
        // sc-stream.localhost. The success response already sets this --
        // omitting it here was the actual bug.
        .header("Access-Control-Allow-Origin", "*")
        .body(message.into_bytes())
        .unwrap()
}

fn extract_track_id(request: &Request<Vec<u8>>) -> Option<i64> {
    // Expected shape: sc-stream://localhost/track/12345
    let path = request.uri().path();
    path.rsplit('/').next()?.parse().ok()
}

pub fn handler(_ctx: UriSchemeContext<'_, tauri::Wry>, request: Request<Vec<u8>>, responder: UriSchemeResponder) {
    let Some(track_id) = extract_track_id(&request) else {
        responder.respond(error_response(StatusCode::BAD_REQUEST, "expected sc-stream://localhost/track/<id>".into()));
        return;
    };

    tauri::async_runtime::spawn(async move {
        let response = fetch_track_audio(track_id).await;
        responder.respond(response);
    });
}

async fn fetch_track_audio(track_id: i64) -> Response<Vec<u8>> {
    let oauth_token = crate::auth::get_stored_token();
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) soundcloud-desktop/0.1")
        .build()
        .expect("failed to build reqwest client");

    let track = match crate::soundcloud::api::get_track(&client, track_id, oauth_token.as_deref()).await {
        Ok(t) => t,
        Err(e) => return error_response(StatusCode::BAD_GATEWAY, format!("failed to load track {track_id}: {e}")),
    };

    if !track.streamable {
        return error_response(StatusCode::FORBIDDEN, format!("track {track_id} is not streamable (private, geo-blocked, or removed)"));
    }

    if track.only_has_unsupported_transcodings() {
        return error_response(
            StatusCode::NOT_IMPLEMENTED,
            format!("track {track_id} is only available as DRM-protected or an unsupported codec -- this app can't play it"),
        );
    }

    let candidates = track.candidate_transcodings();
    if candidates.is_empty() {
        return error_response(StatusCode::BAD_GATEWAY, format!("track {track_id} has no available transcodings"));
    }

    let mut last_error = String::new();
    for transcoding in &candidates {
        let is_hls = transcoding.format.as_ref().and_then(|f| f.protocol.as_deref()) == Some("hls");
        let mime_type = transcoding
            .format
            .as_ref()
            .and_then(|f| f.mime_type.clone())
            .unwrap_or_else(|| "audio/mpeg".to_string());

        let stream_url = match crate::soundcloud::api::resolve_transcoding_url(&client, transcoding, oauth_token.as_deref()).await {
            Ok(url) => url,
            Err(e) => {
                last_error = format!("failed to resolve stream for track {track_id}: {e}");
                continue;
            }
        };

        let bytes_result = if is_hls {
            fetch_hls_audio(&client, &stream_url).await
        } else {
            fetch_progressive_audio(&client, &stream_url).await
        };

        match bytes_result {
            Ok(bytes) if !bytes.is_empty() => {
                return Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", mime_type)
                    .header("Content-Length", bytes.len().to_string())
                    .header("Access-Control-Allow-Origin", "*")
                    .body(bytes)
                    .unwrap();
            }
            Ok(_) => last_error = "resolved stream returned zero bytes".to_string(),
            Err(e) => last_error = e.to_string(),
        }
    }

    error_response(
        StatusCode::BAD_GATEWAY,
        format!("track {track_id}: all {} candidate transcoding(s) failed; last error: {last_error}", candidates.len()),
    )
}

async fn fetch_progressive_audio(client: &reqwest::Client, stream_url: &str) -> anyhow::Result<Vec<u8>> {
    let resp = client.get(stream_url).send().await?;
    if !resp.status().is_success() {
        anyhow::bail!("resolved stream URL returned {}", resp.status());
    }
    Ok(resp.bytes().await?.to_vec())
}

/// Fetches an HLS VOD playlist and concatenates its segments into one byte
/// buffer. Only used for plain (unencrypted) MP3-segment HLS -- see
/// Track::preferred_transcoding for why encrypted/fMP4 HLS never reaches here.
/// MP3 frames are self-synchronizing, so concatenated segments form one
/// valid, seekable MP3 stream.
async fn fetch_hls_audio(client: &reqwest::Client, playlist_url: &str) -> anyhow::Result<Vec<u8>> {
    let base = reqwest::Url::parse(playlist_url)?;
    let playlist_text = client.get(playlist_url).send().await?.error_for_status()?.text().await?;

    let segment_urls: Vec<reqwest::Url> = playlist_text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .filter_map(|line| base.join(line).ok())
        .collect();

    if segment_urls.is_empty() {
        anyhow::bail!("HLS playlist had no segment URLs");
    }

    let mut buf = Vec::new();
    for seg_url in segment_urls {
        let bytes = client.get(seg_url).send().await?.error_for_status()?.bytes().await?;
        buf.extend_from_slice(&bytes);
    }
    Ok(buf)
}

