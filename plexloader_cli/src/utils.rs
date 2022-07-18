mod term;
mod fs;

pub use term::*;
pub use fs::*;

use plexloader_lib::loader::PlexLoader;
use plexloader_lib::utils::fs::deserialize_plex_user;
use anyhow::{Result, Context};

pub fn get_plex_loader() -> Result<PlexLoader> {
    let auth_file_path = get_auth_file_path();
    let plex_user = deserialize_plex_user(auth_file_path)
        .with_context(|| "unable to use previous auth. try logging in again?")?;
    Ok(PlexLoader::new(plex_user)?)
}
