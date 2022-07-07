use clap::{Args};
use plexloader_lib::loader::PlexLoader;

use crate::utils::{get_download_dir};

use anyhow::{Result, Context};

#[derive(Args)]
pub struct Link {
    /// Plex resource link
    link: String,
}

impl Link {
    pub fn handle(self: &Self, plex_loader: PlexLoader) -> Result<()> {
        let download_dir = get_download_dir();
        plex_loader.download_media(&self.link, download_dir)
            .with_context(|| "unable to download from given link")?;
        Ok(())
    }
}
