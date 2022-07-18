use std::path::{PathBuf};
use tauri::api::path::{video_dir};
use plexloader_lib::utils::fs::create_dir;

pub fn init_dirs() -> anyhow::Result<()>{
    let download_dir = get_download_dir();
    create_dir(&download_dir)?;
    Ok(())
}

pub fn get_auth_file_path(app_handle: tauri::AppHandle) -> anyhow::Result<PathBuf> {
    let mut app_dir = app_handle.path_resolver().app_dir()
        .unwrap_or(PathBuf::from("data"));
    create_dir(&app_dir)?;
    app_dir.push("auth.json");
    Ok(app_dir)
}


pub fn get_download_dir() -> PathBuf {
    video_dir()
        .map_or(PathBuf::from("Downloads"), |p| p.join("Plex Downloads"))
}
