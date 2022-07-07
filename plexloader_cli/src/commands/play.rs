use clap::{Args};

use anyhow::{Context};

use super::CommandHandler;
use crate::utils::{get_plex_loader};

#[derive(Args)]
pub struct Play {
    /// Plex resource link
    link: String,
}

impl CommandHandler for Play {
    fn handle(self: &Self) -> anyhow::Result<()> {
        let plex_loader = get_plex_loader()?;
        plex_loader.playback_media(&self.link)
            .with_context(|| "unable to play link")?;
        Ok(())
    }
}
