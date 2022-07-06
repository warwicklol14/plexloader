use clap::Args;

use super::CommandHandler;

use anyhow::{Result, Context};

use crate::utils::{deserialize_plex_user};
use plexloader_lib::loader::PlexLoader;

#[derive(Args)]
pub struct Download {
    /// Link for plex resource
    #[clap(short, long)]
    link: String,
}

impl CommandHandler for Download {
    fn handle(self: &Self) -> Result<()> {
        let plex_user = deserialize_plex_user()
            .with_context(|| "unable to use previous auth. try logging in again?")?;
        let plex_loader = PlexLoader::new(plex_user);
        let req_media_metadata_uri = plex_loader.get_metadata_uri(&self.link)
            .with_context(|| "unable to parse metadata uri")?;
        let req_media_container = plex_loader.get_media(&req_media_metadata_uri)
            .with_context(|| "unable to deserialize media container")?;
        println!("{:?}", req_media_container);
        Ok(())
    }
}
