use plexloader_lib::PlexUser;
use std::fs::File;

pub fn serialize_plex_user(plex_user: PlexUser) -> std::io::Result<()> {
    let plex_user_json_file = File::create("auth.json")?;
    serde_json::to_writer(plex_user_json_file, &plex_user)?;
    Ok(())
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum PlexUserDeserializationError{
    #[error("file couldn't be opened for reading")]
    FileError(#[from] std::io::Error),

    #[error("unable to deserialize json")]
    DeserializationError(#[from] serde_json::Error),
}

pub fn deserialize_plex_user() -> Result<PlexUser, PlexUserDeserializationError> {
    let plex_user_json_file = File::open("auth.json")?;
    match serde_json::from_reader(plex_user_json_file) {
        Ok(plex_user) => Ok(plex_user),
        Err(e) => Err(PlexUserDeserializationError::DeserializationError(e))
    }
}
