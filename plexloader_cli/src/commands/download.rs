use clap::Args;

use anyhow::{Result, Context};

use crate::utils::{deserialize_plex_user};
use plexloader_lib::loader::PlexLoader;

#[derive(Args)]
pub struct Download {
    /// Link for plex resource
    #[clap(short, long)]
    link: String,
}

impl Download {
    pub fn handle(self: &Self) -> Result<()> {
        let plex_user = deserialize_plex_user()
            .with_context(|| "Unable to use previous auth. Maybe try logging in again?")?;
        let plex_loader = PlexLoader::new(plex_user);
        let req_media_metadata_uri = plex_loader.get_metadata_uri(&self.link)
            .with_context(|| "Unable to parse metadata uri")?;
        let req_media_container = plex_loader.get_media(&req_media_metadata_uri)
            .with_context(|| "Unable to deserialize media container")?;
        println!("{:?}", req_media_container);
        Ok(())
    }
}
