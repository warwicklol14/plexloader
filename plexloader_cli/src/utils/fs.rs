use std::path::{PathBuf};
use std::fs::File;
use plexloader_lib::PlexUser;
use directories::{ProjectDirs, UserDirs};
use plexloader_lib::utils::fs::create_dir;

pub fn init_dirs() -> anyhow::Result<()>{
    let data_dir = get_cli_data_dir();
    create_dir(&data_dir)?;
    let download_dir = get_download_dir();
    create_dir(&download_dir)?;
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
        .map_or(PathBuf::from("data"), |project_dirs| project_dirs.data_dir().to_path_buf())
}


pub fn get_download_dir() -> PathBuf {
    UserDirs::new()
        .and_then(|user_dirs: UserDirs| Some(user_dirs.video_dir()?.to_path_buf()) )
        .map(|p| p.join("Plex Downloads"))
        .unwrap_or(PathBuf::from("Downloads"))
}


pub fn deserialize_plex_user() -> std::io::Result<PlexUser> {
    let plex_user_json_file = File::open(get_auth_file_path().as_path())?;
    let plex_user: PlexUser = serde_json::from_reader(plex_user_json_file)?;
    Ok(plex_user)
}


