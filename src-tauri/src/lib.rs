pub mod auth;
pub mod official_oauth;
pub mod playback;
pub mod soundcloud;

use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{Manager, WindowEvent};

const MAIN_WINDOW_LABEL: &str = "main";

fn show_main_window(app: &tauri::AppHandle) {
    if let Some(win) = app.get_webview_window(MAIN_WINDOW_LABEL) {
        let _ = win.show();
        let _ = win.set_focus();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_dialog::init())
        .register_asynchronous_uri_scheme_protocol("sc-stream", playback::handler)
        .setup(|app| {
            // Tray icon so closing the window (see the CloseRequested
            // handler below) can hide it instead of quitting -- playback
            // keeps running in the hidden window, and this is how the user
            // gets back to it or actually exits.
            let show_item = MenuItem::with_id(app, "show", "Show SoundKitten", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_item, &quit_item])?;

            TrayIconBuilder::new()
                .icon(app.default_window_icon().cloned().expect("app has a default window icon"))
                .tooltip("SoundKitten")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => show_main_window(app),
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } = event {
                        show_main_window(tray.app_handle());
                    }
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                if window.label() == MAIN_WINDOW_LABEL {
                    // Hide instead of quitting -- playback continues in the
                    // background, reachable again from the tray icon. The
                    // tray menu's "Quit" is the only thing that actually
                    // exits.
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            auth::is_logged_in,
            auth::logout,
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
            official_oauth::start_official_login,
            official_oauth::is_official_connected,
            official_oauth::disconnect_official_login,
            official_oauth::sc_like_track_v2,
            official_oauth::sc_unlike_track_v2,
            official_oauth::sc_like_playlist_v2,
            official_oauth::sc_unlike_playlist_v2,
            official_oauth::sc_follow_user_v2,
            official_oauth::sc_unfollow_user_v2,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
