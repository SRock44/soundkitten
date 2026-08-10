pub mod auth;
pub mod official_oauth;
pub mod playback;
pub mod soundcloud;

use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{Manager, WebviewUrl, WebviewWindowBuilder, WindowEvent};

const MAIN_WINDOW_LABEL: &str = "main";
const MINI_PLAYER_WINDOW_LABEL: &str = "mini-player";

/// WebView2's own native right-click menu (Back/Forward/Reload/Save as/
/// Print/Inspect) is enabled by default and does NOT reliably get
/// suppressed by a page-level `contextmenu` handler's `preventDefault()`
/// -- confirmed live, the app's custom per-track context menu
/// (TrackRow.svelte) never got a chance to render, the native WebView2
/// menu won every time regardless.
///
/// The first fix attempted here was `ICoreWebView2Settings.
/// SetAreDefaultContextMenusEnabled(false)` -- WRONG, and confirmed live
/// to make things worse: per Microsoft's own WebView2 docs
/// (https://learn.microsoft.com/en-us/microsoft-edge/webview2/how-to/context-menus),
/// "If AreDefaultContextMenusEnabled is set to False ... the
/// ContextMenuRequested event won't be raised" -- it doesn't just hide
/// the native menu, it tears down the whole native context-menu request
/// pipeline, and empirically that also meant the page's own
/// `contextmenu`/right-button `mousedown` events stopped arriving at all
/// (right-click did nothing whatsoever, not even the app's own menu).
///
/// The actual correct API for "build your own context-menu UI" (this
/// app's exact use case, and Microsoft's own docs frame it this way) is
/// `ICoreWebView2_11::add_ContextMenuRequested`: a native hook that fires
/// on every right-click independent of the DOM's own `contextmenu`
/// event, whose args expose `Handled` -- set it `true` to suppress only
/// WebView2's native menu UI, leaving the page's own JS event handling
/// (TrackRow.svelte's `oncontextmenu`/`onmousedown`) completely
/// untouched. No-op on macOS/Linux (WKWebView/WebKitGTK don't have this
/// native-menu-vs-DOM-event entanglement, and page-level preventDefault
/// already works normally there).
#[cfg(windows)]
fn disable_default_context_menu(window: &tauri::WebviewWindow) {
    let _ = window.with_webview(|webview| {
        use webview2_com::ContextMenuRequestedEventHandler;
        use webview2_com::Microsoft::Web::WebView2::Win32::ICoreWebView2_11;
        use windows::core::Interface;

        unsafe {
            let Ok(core) = webview.controller().CoreWebView2() else { return };
            let Ok(core11) = core.cast::<ICoreWebView2_11>() else { return };
            let handler = ContextMenuRequestedEventHandler::create(Box::new(|_sender, args| {
                if let Some(args) = args {
                    args.SetHandled(true)?;
                }
                Ok(())
            }));
            // Token intentionally not retained -- this hook lives for the
            // whole life of the window, same as the window itself never
            // explicitly unregistering its close handler.
            let mut token = Default::default();
            let _ = core11.add_ContextMenuRequested(&handler, &mut token);
        }
    });
}

#[cfg(not(windows))]
fn disable_default_context_menu(_window: &tauri::WebviewWindow) {}

fn show_main_window(app: &tauri::AppHandle) {
    if let Some(win) = app.get_webview_window(MAIN_WINDOW_LABEL) {
        let _ = win.show();
        let _ = win.set_focus();
    }
}

/// Opens the mini player, a second window pointed at the same app entry
/// (this app has no SvelteKit sub-routes, everything is one page with
/// internal view state by design -- +page.svelte checks its own window
/// label at startup to decide which UI to render, see src/routes/+page.svelte).
/// Idempotent: focuses the existing window instead of creating a duplicate
/// -- the window is only ever hidden, never destroyed (see the
/// CloseRequested handler below), so after the very first open this
/// always just shows the same already-synced window. Hides the main
/// window either way, they're meant to be alternate views, not both open
/// at once.
///
/// Must be `async fn`: a plain sync command runs on the main/event-loop
/// thread, and WebviewWindowBuilder::build() on Windows has to dispatch
/// window creation back onto that same main thread and block waiting for
/// it -- called from the main thread itself, that wait never resolves,
/// which deadlocks the whole app (observed live: the new window painted
/// nothing and the main window stopped responding entirely). `async fn`
/// commands run off the main thread, avoiding that -- same pattern
/// already used by auth::start_login/official_oauth::start_official_login
/// for their own window creation.
#[tauri::command]
async fn open_mini_player(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(win) = app.get_webview_window(MINI_PLAYER_WINDOW_LABEL) {
        let _ = win.show();
        let _ = win.set_focus();
    } else {
        // Deliberately NOT always_on_top -- that pinned it above every
        // other application's windows, not just SoundKitten's own, which
        // is not what "small floating widget" should mean. Resizable so
        // users who want a bigger widget can drag it larger -- the
        // frontend layout (MiniPlayer.svelte) is flex-based specifically
        // so it reflows sanely rather than just clipping.
        let win = WebviewWindowBuilder::new(&app, MINI_PLAYER_WINDOW_LABEL, WebviewUrl::App("index.html".into()))
            .title("SoundKitten")
            .inner_size(320.0, 148.0)
            .min_inner_size(260.0, 120.0)
            .resizable(true)
            .decorations(false)
            .build()
            .map_err(|e| e.to_string())?;
        disable_default_context_menu(&win);
    }
    if let Some(main) = app.get_webview_window(MAIN_WINDOW_LABEL) {
        let _ = main.hide();
    }
    Ok(())
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
            if let Some(main) = app.get_webview_window(MAIN_WINDOW_LABEL) {
                disable_default_context_menu(&main);
            }

            // Tray icon so closing the window (see the CloseRequested
            // handler below) can hide it instead of quitting -- playback
            // keeps running in the hidden window, and this is how the user
            // gets back to it or actually exits.
            let show_item = MenuItem::with_id(app, "show", "Show SoundKitten", true, None::<&str>)?;
            let mini_player_item = MenuItem::with_id(app, "mini_player", "Mini Player", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_item, &mini_player_item, &quit_item])?;

            TrayIconBuilder::new()
                .icon(app.default_window_icon().cloned().expect("app has a default window icon"))
                .tooltip("SoundKitten")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => show_main_window(app),
                    "mini_player" => {
                        let app = app.clone();
                        tauri::async_runtime::spawn(async move {
                            let _ = open_mini_player(app).await;
                        });
                    }
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
                } else if window.label() == MINI_PLAYER_WINDOW_LABEL {
                    // Hide, not destroy -- same reasoning as the main
                    // window above, plus one more: destroying it meant
                    // every reopen was a full fresh webview load (blank
                    // "Nothing playing" until the first state sync
                    // round-trip landed, visible as the mini player
                    // "restarting" on every switch back from the main
                    // window). Hidden, its JS keeps running and stays
                    // synced the whole time (see PlayerBar.svelte's
                    // emitters), so reopening is instant and already
                    // correct -- open_mini_player's existing
                    // show()+set_focus() branch for "window already
                    // exists" now covers every reopen, not just a
                    // same-session double-click.
                    api.prevent_close();
                    let _ = window.hide();
                    show_main_window(&window.app_handle().clone());
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            open_mini_player,
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
            soundcloud::commands::sc_search_all,
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
            official_oauth::sc_create_playlist_v2,
            official_oauth::sc_update_playlist_v2,
            official_oauth::sc_delete_playlist_v2,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
