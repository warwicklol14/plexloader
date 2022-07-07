use clap::{Args};
use plexloader_lib::loader::PlexLoader;
use crate::utils::{info, success};

use anyhow::{Result, Context};

#[derive(Args)]
pub struct Link {
    /// Plex resource link
    link: String,
}

impl Link {
    pub fn handle(self: &Self, plex_loader: PlexLoader) -> Result<()> {
        let media_resource = plex_loader.get_media_resource(&self.link)
            .with_context(|| "unable to get media")?;
        println!("{}", success().apply_to("These are the following media found from the given link:"));
        println!("\t{}", info().apply_to(&media_resource.title));
        Ok(())
    }
}
