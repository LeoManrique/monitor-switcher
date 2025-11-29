//! Monitor Switcher - Save and restore Windows display configurations.

mod ccd;
mod profile;

use ccd::{get_display_settings, set_display_settings, turn_off_monitors as ccd_turn_off, match_adapter_ids, get_additional_info_for_modes};
use profile::{settings_to_profile, profile_to_settings, list_profiles as storage_list, save_profile as storage_save, load_profile as storage_load, delete_profile as storage_delete, profile_exists as storage_exists, get_profile_details as storage_get_details, MonitorDetails};

use serde::Serialize;
use tauri::{
    AppHandle, Manager, WebviewUrl, WebviewWindowBuilder, Wry,
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    menu::{Menu, MenuItem, IconMenuItem, Submenu, PredefinedMenuItem},
    image::Image,
};
use std::path::PathBuf;
use log::{info, error};

// ============================================================================
// Types for Frontend
// ============================================================================

/// Profile with detailed monitor information.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileDetails {
    pub name: String,
    pub monitors: Vec<MonitorDetails>,
}

// ============================================================================
// Tauri Commands
// ============================================================================

#[tauri::command]
async fn list_profiles() -> Result<Vec<String>, String> {
    storage_list()
}

#[tauri::command]
async fn list_profiles_with_details() -> Result<Vec<ProfileDetails>, String> {
    let names = storage_list()?;
    let mut profiles = Vec::new();

    for name in names {
        match storage_get_details(&name) {
            Ok(monitors) => {
                profiles.push(ProfileDetails { name, monitors });
            }
            Err(e) => {
                log::warn!("Failed to get details for profile '{}': {}", name, e);
                // Include profile with empty monitors on error
                profiles.push(ProfileDetails { name, monitors: Vec::new() });
            }
        }
    }

    Ok(profiles)
}

#[tauri::command]
async fn save_profile(app: AppHandle, name: String) -> Result<(), String> {
    info!("Saving profile: {}", name);

    // Get current display settings
    let settings = get_display_settings(true)?;

    // Get additional monitor info
    let additional_info = get_additional_info_for_modes(&settings.mode_info_array);

    // Convert to profile format
    let profile = settings_to_profile(&settings, &additional_info);

    // Save to disk
    storage_save(&name, &profile)?;

    // Refresh tray menu to show new profile
    let _ = refresh_tray_menu(&app);

    info!("Profile '{}' saved successfully", name);
    Ok(())
}

#[tauri::command]
async fn load_profile(name: String) -> Result<(), String> {
    info!("Loading profile: {}", name);

    // Load profile from disk
    let profile = storage_load(&name)?;

    // Convert to CCD settings
    let (mut settings, additional_info) = profile_to_settings(&profile);

    // Match adapter IDs to current system
    match_adapter_ids(&mut settings, &additional_info)?;

    // Apply settings
    set_display_settings(&mut settings)?;

    info!("Profile '{}' loaded successfully", name);
    Ok(())
}

#[tauri::command]
async fn delete_profile(app: AppHandle, name: String) -> Result<(), String> {
    info!("Deleting profile: {}", name);
    storage_delete(&name)?;

    // Refresh tray menu to remove deleted profile
    let _ = refresh_tray_menu(&app);

    info!("Profile '{}' deleted successfully", name);
    Ok(())
}

#[tauri::command]
async fn profile_exists(name: String) -> Result<bool, String> {
    storage_exists(&name)
}

#[tauri::command]
async fn turn_off_monitors() -> Result<(), String> {
    info!("Turning off monitors");
    ccd_turn_off()
}

#[tauri::command]
async fn open_save_dialog(app: AppHandle) -> Result<(), String> {
    open_save_popup(&app);
    Ok(())
}

// ============================================================================
// Popup Window
// ============================================================================

fn open_save_popup(app: &AppHandle<Wry>) {
    // If popup already exists, just focus it
    if let Some(window) = app.get_webview_window("save-popup") {
        let _ = window.set_focus();
        return;
    }

    // Calculate dynamic height based on number of profiles
    // Base height: title bar (40) + input section (70) + buttons (50) + padding (30) = 190
    // Per profile: ~39px each (compact list style)
    // Min height when no profiles: 165
    // Max height: 350 (to avoid huge windows)
    let profile_count = storage_list().unwrap_or_default().len();
    let base_height = 165.0_f64;
    let per_profile_height = 39.0_f64;
    let section_header_height = if profile_count > 0 { 38.0 } else { 0.0 };
    let calculated_height = base_height + section_header_height + (profile_count as f64 * per_profile_height);
    let popup_height = calculated_height.min(350.0);

    // Create popup window
    let app_clone = app.clone();
    match WebviewWindowBuilder::new(
        app,
        "save-popup",
        WebviewUrl::App("popup.html".into()),
    )
    .title("Save Profile")
    .inner_size(300.0, popup_height)
    .resizable(false)
    .maximizable(false)
    .minimizable(false)
    .decorations(false)
    .center()
    .focused(true)
    .build()
    {
        Ok(window) => {
            // Refresh tray menu when popup closes (profile may have been saved)
            window.on_window_event(move |event| {
                if let tauri::WindowEvent::Destroyed = event {
                    let _ = refresh_tray_menu(&app_clone);
                }
            });
        }
        Err(e) => {
            error!("Failed to create save popup: {}", e);
        }
    }
}

// ============================================================================
// System Tray
// ============================================================================

/// Load a menu icon from the icons/menu directory
fn load_menu_icon(app: &AppHandle<Wry>, name: &str) -> Option<Image<'static>> {
    let resource_path: PathBuf = app
        .path()
        .resource_dir()
        .ok()?
        .join("icons")
        .join("menu")
        .join(format!("{}.ico", name));

    Image::from_path(&resource_path).ok()
}

fn build_tray_menu(app: &AppHandle<Wry>) -> Result<Menu<Wry>, tauri::Error> {
    let profiles = storage_list().unwrap_or_default();

    // Load icons
    let monitor_icon = load_menu_icon(app, "monitor");
    let monitor_delete_icon = load_menu_icon(app, "monitor-delete");
    let save_icon = load_menu_icon(app, "save");
    let delete_icon = load_menu_icon(app, "delete");
    let power_icon = load_menu_icon(app, "power");
    let window_icon = load_menu_icon(app, "window");
    let exit_icon = load_menu_icon(app, "exit");

    // Build Load Profile submenu
    let load_submenu = {
        let submenu = Submenu::with_id_and_items(app, "load_submenu", "Load Profile", true, &[])?;
        submenu.set_icon(monitor_icon.clone())?;
        if profiles.is_empty() {
            submenu.append(&MenuItem::with_id(app, "no_profiles", "(No profiles)", false, None::<&str>)?)?;
        } else {
            for profile in &profiles {
                submenu.append(&IconMenuItem::with_id(
                    app,
                    format!("load_{}", profile),
                    profile,
                    true,
                    monitor_icon.clone(),
                    None::<&str>,
                )?)?;
            }
        }
        submenu
    };

    // Build Save Profile submenu
    let save_submenu = {
        let submenu = Submenu::with_id_and_items(app, "save_submenu", "Save Profile", true, &[])?;
        submenu.set_icon(save_icon.clone())?;
        submenu.append(&IconMenuItem::with_id(app, "save_new", "New Profile...", true, save_icon.clone(), None::<&str>)?)?;
        if !profiles.is_empty() {
            submenu.append(&PredefinedMenuItem::separator(app)?)?;
            for profile in &profiles {
                submenu.append(&IconMenuItem::with_id(
                    app,
                    format!("save_{}", profile),
                    profile,
                    true,
                    monitor_icon.clone(),
                    None::<&str>,
                )?)?;
            }
        }
        submenu
    };

    // Build Delete Profile submenu
    let delete_submenu = {
        let submenu = Submenu::with_id_and_items(app, "delete_submenu", "Delete Profile", !profiles.is_empty(), &[])?;
        submenu.set_icon(delete_icon.clone())?;
        if profiles.is_empty() {
            submenu.append(&MenuItem::with_id(app, "no_profiles_delete", "(No profiles)", false, None::<&str>)?)?;
        } else {
            for profile in &profiles {
                submenu.append(&IconMenuItem::with_id(
                    app,
                    format!("delete_{}", profile),
                    profile,
                    true,
                    monitor_delete_icon.clone(),
                    None::<&str>,
                )?)?;
            }
        }
        submenu
    };

    // Build main menu
    let menu = Menu::new(app)?;
    menu.append(&load_submenu)?;
    menu.append(&save_submenu)?;
    menu.append(&delete_submenu)?;
    menu.append(&PredefinedMenuItem::separator(app)?)?;
    menu.append(&IconMenuItem::with_id(app, "turn_off", "Turn Off All Monitors", true, power_icon, None::<&str>)?)?;
    menu.append(&PredefinedMenuItem::separator(app)?)?;
    menu.append(&IconMenuItem::with_id(app, "open_window", "Open Window", true, window_icon, None::<&str>)?)?;
    menu.append(&IconMenuItem::with_id(app, "quit", "Exit", true, exit_icon, None::<&str>)?)?;

    Ok(menu)
}

fn setup_tray(app: &AppHandle<Wry>) -> Result<(), Box<dyn std::error::Error>> {
    let menu = build_tray_menu(app)?;

    let _tray = TrayIconBuilder::with_id("main")
        .icon(app.default_window_icon().cloned().unwrap())
        .menu(&menu)
        .tooltip("Monitor Switcher")
        .on_menu_event(move |app, event| {
            let id = event.id().as_ref();

            if id.starts_with("load_") {
                let profile_name = &id[5..];
                let name = profile_name.to_string();
                tauri::async_runtime::spawn(async move {
                    if let Err(e) = load_profile(name.clone()).await {
                        error!("Failed to load profile '{}': {}", name, e);
                    }
                });
            } else if id.starts_with("save_") && id != "save_new" {
                let profile_name = &id[5..];
                let app_clone = app.clone();
                let name = profile_name.to_string();
                tauri::async_runtime::spawn(async move {
                    if let Err(e) = save_profile(app_clone, name.clone()).await {
                        error!("Failed to save profile '{}': {}", name, e);
                    }
                });
            } else if id.starts_with("delete_") {
                let profile_name = &id[7..];
                let app_clone = app.clone();
                let name = profile_name.to_string();
                tauri::async_runtime::spawn(async move {
                    if let Err(e) = delete_profile(app_clone, name.clone()).await {
                        error!("Failed to delete profile '{}': {}", name, e);
                    }
                });
            } else {
                match id {
                    "save_new" => {
                        // Open popup window for new profile
                        open_save_popup(app);
                    }
                    "turn_off" => {
                        tauri::async_runtime::spawn(async {
                            if let Err(e) = turn_off_monitors().await {
                                error!("Failed to turn off monitors: {}", e);
                            }
                        });
                    }
                    "open_window" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                }
            }
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app)?;

    Ok(())
}

fn refresh_tray_menu(app: &AppHandle<Wry>) -> Result<(), Box<dyn std::error::Error>> {
    // Rebuild the menu with updated profiles
    let menu = build_tray_menu(app)?;

    // Get the tray icon and update its menu
    if let Some(tray) = app.tray_by_id("main") {
        tray.set_menu(Some(menu))?;
    }

    Ok(())
}

// ============================================================================
// App Entry Point
// ============================================================================

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            // Focus the main window when another instance is launched
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .setup(|app| {
            // Setup system tray
            if let Err(e) = setup_tray(app.handle()) {
                error!("Failed to setup tray: {}", e);
            }

            // Hide window on close instead of quitting
            let window = app.get_webview_window("main").unwrap();
            let window_clone = window.clone();
            window.on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    let _ = window_clone.hide();
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            list_profiles,
            list_profiles_with_details,
            save_profile,
            load_profile,
            delete_profile,
            profile_exists,
            turn_off_monitors,
            open_save_dialog,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
