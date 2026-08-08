//! `sc-stream://` custom protocol: proxies authenticated SoundCloud audio
//! bytes to the frontend's <audio> element, since <audio src> can't attach
//! the Authorization header or client_id itself. Streaming only -- bytes are
//! piped straight to the webview, never written to a user-accessible file.

use tauri::http::{Request, Response, StatusCode};
use tauri::{UriSchemeContext, UriSchemeResponder};

fn error_response(status: StatusCode, message: String) -> Response<Vec<u8>> {
    Response::builder()
        .status(status)
        .header("Content-Type", "text/plain")
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

    let stream_url = match crate::soundcloud::api::resolve_stream_url(&client, &track, oauth_token.as_deref()).await {
        Ok(url) => url,
        Err(e) => return error_response(StatusCode::BAD_GATEWAY, format!("failed to resolve stream for track {track_id}: {e}")),
    };

    let mime_type = track
        .preferred_transcoding()
        .and_then(|t| t.format.as_ref())
        .and_then(|f| f.mime_type.clone())
        .unwrap_or_else(|| "audio/mpeg".to_string());

    let audio_resp = match client.get(&stream_url).send().await {
        Ok(r) => r,
        Err(e) => return error_response(StatusCode::BAD_GATEWAY, format!("failed to fetch resolved stream: {e}")),
    };
    if !audio_resp.status().is_success() {
        return error_response(StatusCode::BAD_GATEWAY, format!("resolved stream URL returned {}", audio_resp.status()));
    }

    let bytes = match audio_resp.bytes().await {
        Ok(b) => b.to_vec(),
        Err(e) => return error_response(StatusCode::BAD_GATEWAY, format!("failed to read audio bytes: {e}")),
    };
    if bytes.is_empty() {
        return error_response(StatusCode::BAD_GATEWAY, "resolved stream returned zero bytes".into());
    }

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", mime_type)
        .header("Content-Length", bytes.len().to_string())
        .header("Access-Control-Allow-Origin", "*")
        .body(bytes)
        .unwrap()
}

