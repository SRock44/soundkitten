pub mod auth;
pub mod playback;
pub mod soundcloud;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_dialog::init())
        .register_asynchronous_uri_scheme_protocol("sc-stream", playback::handler)
        .invoke_handler(tauri::generate_handler![
            auth::is_logged_in,
            auth::logout,
            auth::set_manual_token,
            auth::start_login,
            auth::verify_auth,
            soundcloud::set_client_id_override,
            soundcloud::clear_client_id_override,
            soundcloud::get_client_id_override,
            soundcloud::commands::sc_search,
            soundcloud::commands::sc_resolve,
            soundcloud::commands::sc_likes,
            soundcloud::commands::sc_playlists,
            soundcloud::commands::sc_me,
            soundcloud::commands::sc_user_profile,
            soundcloud::commands::sc_user_tracks,
            soundcloud::commands::sc_search_users,
            soundcloud::commands::sc_feed,
            soundcloud::commands::sc_like_track,
            soundcloud::commands::sc_unlike_track,
            soundcloud::commands::sc_repost_track,
            soundcloud::commands::sc_unrepost_track,
            soundcloud::commands::sc_track_comments,
            soundcloud::commands::sc_post_comment,
            soundcloud::commands::sc_user_reposts,
            soundcloud::commands::sc_user_playlists,
            soundcloud::commands::sc_user_followers,
            soundcloud::commands::sc_user_followings,
            soundcloud::commands::sc_my_followings_ids,
            soundcloud::commands::sc_user_comments,
            soundcloud::commands::sc_playlist,
            soundcloud::commands::sc_mixed_selections,
            soundcloud::commands::sc_system_playlist_tracks,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
