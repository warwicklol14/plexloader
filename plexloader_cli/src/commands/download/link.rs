use clap::{Args};
use plexloader_lib::loader::PlexLoader;

use anyhow::{Result, Context};

#[derive(Args)]
pub struct Link {
    /// Plex resource link
    #[clap(short, long)]
    link: String,
}

impl Link {
    pub fn handle(self: &Self, plex_loader: PlexLoader) -> Result<()> {
        let media_resource = plex_loader.get_media_resource(&self.link)?;
        println!("{:?}", media_resource);
        Ok(())
    }
}
