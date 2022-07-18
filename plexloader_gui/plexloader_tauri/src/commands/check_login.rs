use crate::utils::fs::{init_dirs, get_auth_file_path};
use crate::error::error_to_string;
use plexloader_lib::utils::fs::deserialize_plex_user;
use plexloader_lib::loader::PlexLoader;
use anyhow::Context;
use crate::AppState;

#[tauri::command]
pub async fn check_login(state: tauri::State<'_, AppState>, app_handle: tauri::AppHandle) -> Result<(), String> {
    init_dirs()
        .map_err(|e| error_to_string(e.into()))?;

    let auth_file_path = get_auth_file_path(app_handle)
        .with_context(|| "unable to get get auth file path")
        .map_err(|e| error_to_string(e.into()))?;
    let plex_user = deserialize_plex_user(auth_file_path)
        .with_context(|| "unable to deserialize plex user")
        .map_err(|e| error_to_string(e.into()))?;

    let plexloader = PlexLoader::new(plex_user)
        .map_err(|e| error_to_string(e.into()))?;

    let mut plex_loader_guard = state.plex_loader.lock()
        .unwrap();

    *plex_loader_guard = Some(plexloader);

    Ok(())
}
