use plexloader_lib::PlexUser;
use std::fs::File;
use std::path::{PathBuf, Path};
use directories::ProjectDirs;

pub fn serialize_plex_user(plex_user: PlexUser) -> std::io::Result<()> {
    let plex_user_json_file = File::create("auth.json")?;
    serde_json::to_writer(plex_user_json_file, &plex_user)?;
    Ok(())
}

pub fn get_cli_data_dir() -> PathBuf {
    if let Some(project_dirs) = ProjectDirs::from("com", "warwick", "plexloader_cli") {
        project_dirs.data_dir().to_path_buf()
    }
    else {
        Path::new("./data").to_path_buf()
    }
}

use thiserror::Error;

#[derive(Error, Debug)]
#[error("can't load auth details")]
pub enum PlexUserDeserializationError {
    #[error("file couldn't be opened for reading")]
    FileError(#[from] std::io::Error),

    #[error("unable to deserialize json")]
    DeserializationError(#[from] serde_json::Error),
}

pub fn deserialize_plex_user() -> Result<PlexUser, PlexUserDeserializationError> {
    let mut plex_user_json_file_path = get_cli_data_dir();
    plex_user_json_file_path.push("auth.json");
    let plex_user_json_file = File::open(plex_user_json_file_path.as_path())?;
    serde_json::from_reader(plex_user_json_file)
        .map_err(|e| PlexUserDeserializationError::DeserializationError(e))
}

pub fn print_err(r: anyhow::Result<()>) {
    if let Err(e) = r {
        eprintln!("Error: {e}");
        e.chain()
            .skip(1)
            .for_each(|cause| eprintln!("because: {}", cause));
    }
}
