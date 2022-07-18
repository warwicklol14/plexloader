use crate::error::error_to_string;
use plexloader_lib::loader::PlexLoader;
use plexloader_lib::loader::login::plex_login_through_credentials;
use plexloader_lib::utils::fs::serialize_plex_user;
use crate::utils::fs::get_auth_file_path;
use anyhow::Context;
use crate::AppState;

#[tauri::command]
pub async fn login(state: tauri::State<'_, AppState>, app_handle: tauri::AppHandle, username: &str, password: &str) -> Result<(), String> {
    let plex_user = plex_login_through_credentials(username, password)
        .map_err(|e| error_to_string(e.into()))?;

    let auth_file_path = get_auth_file_path(app_handle)
        .with_context(|| "unable to get get auth file path")
        .map_err(|e| error_to_string(e.into()))?;
    serialize_plex_user(&plex_user, auth_file_path)
        .with_context(|| "unable to serialize plex user")
        .map_err(|e| error_to_string(e.into()))?;

    let plex_loader = PlexLoader::new(plex_user)
        .map_err(|e| error_to_string(e.into()))?;

    let mut plex_loader_guard = state.plex_loader.lock()
        .unwrap();

    *plex_loader_guard = Some(plex_loader);

    Ok(())
}
