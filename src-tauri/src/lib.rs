pub mod auth;
pub mod soundcloud;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            auth::is_logged_in,
            auth::logout,
            auth::set_manual_token,
            auth::start_login,
            auth::verify_auth,
            soundcloud::set_client_id_override,
            soundcloud::commands::sc_search,
            soundcloud::commands::sc_resolve,
            soundcloud::commands::sc_likes,
            soundcloud::commands::sc_playlists,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
