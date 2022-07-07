mod term;
mod fs;

pub use term::*;
pub use fs::*;

use plexloader_lib::loader::PlexLoader;
use anyhow::{Result, Context};

pub fn get_plex_loader() -> Result<PlexLoader> {
    let plex_user = deserialize_plex_user()
        .with_context(|| "unable to use previous auth. try logging in again?")?;
    Ok(PlexLoader::new(plex_user))
}
