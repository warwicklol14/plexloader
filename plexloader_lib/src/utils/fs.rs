use std::path::{Path, PathBuf};
use std::fs::create_dir_all;
use crate::DirectoryCreationError;
use crate::PlexUser;
use std::fs::File;

pub fn create_dir(dir_path: &Path) -> Result<(), DirectoryCreationError> {
    if !dir_path.exists() {
        create_dir_all(dir_path)?;
    }
    Ok(())
}


pub fn serialize_plex_user(plex_user: &PlexUser, auth_file_path: PathBuf) -> std::io::Result<()> {
    let plex_user_json_file = File::create(&auth_file_path)?;
    serde_json::to_writer(plex_user_json_file, &plex_user)?;
    Ok(())
}

pub fn deserialize_plex_user(auth_file_path: PathBuf) -> std::io::Result<PlexUser> {
    let plex_user_json_file = File::open(&auth_file_path)?;
    let plex_user: PlexUser = serde_json::from_reader(plex_user_json_file)?;
    Ok(plex_user)
}
