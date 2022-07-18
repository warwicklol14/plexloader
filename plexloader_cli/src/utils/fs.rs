use std::path::{PathBuf};
use directories::{ProjectDirs, UserDirs};
use plexloader_lib::utils::fs::create_dir;

pub fn init_dirs() -> anyhow::Result<()>{
    let data_dir = get_cli_data_dir();
    create_dir(&data_dir)?;
    let download_dir = get_download_dir();
    create_dir(&download_dir)?;
    Ok(())
}

pub fn get_cli_data_dir() -> PathBuf {
    ProjectDirs::from("com", "warwick", "plexloader_cli")
        .map_or(PathBuf::from("data"), |project_dirs| project_dirs.data_dir().to_path_buf())
}

pub fn get_auth_file_path() -> PathBuf{
    let mut data_dir_path = get_cli_data_dir();
    data_dir_path.push("auth.json");
    data_dir_path
}


pub fn get_download_dir() -> PathBuf {
    UserDirs::new()
        .and_then(|user_dirs: UserDirs| Some(user_dirs.video_dir()?.to_path_buf()) )
        .map(|p| p.join("Plex Downloads"))
        .unwrap_or(PathBuf::from("Downloads"))
}
