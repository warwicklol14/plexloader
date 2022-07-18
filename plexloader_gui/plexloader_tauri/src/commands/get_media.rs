use crate::{AppState};
use plexloader_lib::PlexMediaResource;
use crate::error::error_to_string;

#[tauri::command]
pub async fn get_media(app_state: tauri::State<'_, AppState>, media_link: &str) -> Result<PlexMediaResource, String> {
    let plex_loader_guard = app_state.plex_loader.lock()
        .unwrap();

    if let Some(plex_loader) = &*plex_loader_guard {
        let media_resource = plex_loader.get_media_resource(media_link)
            .map_err(|e| error_to_string(e.into()));
        return media_resource;
    }

    Err(String::from("unable to get media, no plex loader"))
}

