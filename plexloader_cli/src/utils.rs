use plexloader_lib::PlexUser;
use std::fs::File;
use std::fs::create_dir_all;
use std::path::{PathBuf};
use directories::{ProjectDirs, UserDirs};

pub fn init_dirs() -> anyhow::Result<()>{
    let data_dir = get_cli_data_dir();
    if !&data_dir.exists() {
        create_dir_all(data_dir)?;
    }
    let download_dir = get_download_dir();
    if !&download_dir.exists() {
        create_dir_all(download_dir)?;
    }
    Ok(())
}

fn get_auth_file_path() -> PathBuf {
    get_cli_data_dir().join("auth.json")
}

pub fn serialize_plex_user(plex_user: PlexUser) -> std::io::Result<()> {
    let auth_file_path = get_auth_file_path();
    let plex_user_json_file = File::create(auth_file_path.as_path())?;
    serde_json::to_writer(plex_user_json_file, &plex_user)?;
    Ok(())
}

pub fn get_cli_data_dir() -> PathBuf {
    ProjectDirs::from("com", "warwick", "plexloader_cli")
        .map_or(PathBuf::from("./data"), |project_dirs| project_dirs.data_dir().to_path_buf())
}


pub fn get_download_dir() -> PathBuf {
    UserDirs::new()
        .and_then(|user_dirs: UserDirs| Some(user_dirs.video_dir()?.to_path_buf()) )
        .map(|p| p.join("Plex Downloads"))
        .unwrap_or(PathBuf::from("./Downloads"))
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
    let plex_user_json_file = File::open(get_auth_file_path().as_path())?;
    serde_json::from_reader(plex_user_json_file)
        .map_err(|e| PlexUserDeserializationError::DeserializationError(e))
}

pub fn print_err(r: anyhow::Result<()>) {
    if let Err(e) = r {
        eprintln!("Error: {e}");
        e.chain()
            .skip(1)
            .for_each(|cause| eprintln!("because: {}", cause));
        std::process::exit(1);
    }
}
