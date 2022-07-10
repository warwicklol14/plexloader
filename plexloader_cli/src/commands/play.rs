use clap::{Args};
use dialoguer::{
    Select,
    theme::ColorfulTheme
};
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
        let media_resources = plex_loader.get_media_resources(&self.link)
            .with_context(|| "unable to get media")?;
        if media_resources.len() == 1 {
            plex_loader.playback_media(&media_resources[0])
                .with_context(|| "unable to play link")?;
        }
        else {
            let chosen_media = Select::with_theme(&ColorfulTheme::default())
                .items(&media_resources)
                .with_prompt("Multiple media were found. Select the one you want to play")
                .report(false)
                .interact()?;
            plex_loader.playback_media(&media_resources[chosen_media])
                .with_context(|| "unable to play link")?;
        }
        Ok(())
    }
}
