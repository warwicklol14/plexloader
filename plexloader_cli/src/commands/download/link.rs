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
        let req_media_metadata_uri = plex_loader.get_metadata_uri(&self.link)
            .with_context(|| "unable to parse metadata uri")?;
        let req_media_container = plex_loader.get_media(&req_media_metadata_uri)
            .with_context(|| "unable to deserialize media container")?;
        println!("{:?}", req_media_container);
        Ok(())
    }
}
