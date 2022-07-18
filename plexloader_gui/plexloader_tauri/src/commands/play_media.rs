use plexloader_lib::PlexMediaResource;
use plexloader_lib::loader::player::play_media_mpv;

use crate::error::error_to_string;

#[tauri::command]
pub async fn play_media(media_resource: PlexMediaResource) -> Result<(), String> {
    match media_resource {
        PlexMediaResource::VideoResource(videos) => {
            let mut child = play_media_mpv(&videos[0].resource_path, &videos[0].title, &videos[0].access_token)
                .map_err(|e| error_to_string(e.into()))?;
            child.wait()
                .map_err(|e| error_to_string(e.into()))?;
        },
        PlexMediaResource::DirectoryResource(directories) => {
            let mut child = play_media_mpv(&directories[0].children[0].resource_path, &directories[0].children[0].title, &directories[0].access_token)
                .map_err(|e| error_to_string(e.into()))?;
            child.wait()
                .map_err(|e| error_to_string(e.into()))?;
        }
    }
    Ok(())
}
