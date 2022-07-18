#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

fn main() {
  tauri::Builder::default()
    .manage(AppState {
        plex_loader: Mutex::new(None)
    })
    .invoke_handler(tauri::generate_handler![
        commands::check_login::check_login,
        commands::login::login,
        commands::get_media::get_media,
        commands::play_media::play_media,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

use plexloader_lib::loader::PlexLoader;
use std::sync::Mutex;

pub struct AppState {
    pub plex_loader: Mutex<Option<PlexLoader>>,
}

mod error;
mod commands;
mod utils;
