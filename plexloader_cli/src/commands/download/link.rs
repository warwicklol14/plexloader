use clap::{Args};
use plexloader_lib::loader::PlexLoader;

use crate::utils::{get_download_dir};

use anyhow::{Result, Context};
use dialoguer::{
    MultiSelect,
    theme::ColorfulTheme
};

#[derive(Args)]
pub struct Link {
    /// Plex resource link
    link: String,
}

impl Link {
    pub fn handle(self: &Self, plex_loader: PlexLoader) -> Result<()> {
        let download_dir = get_download_dir();
        let media_resources = plex_loader.get_media_resources(&self.link)
            .with_context(|| "unable to get media")?;
        if media_resources.len() == 1 {
            plex_loader.download_media(&media_resources[0], &download_dir)
                .with_context(|| "unable to download from given link")?;
        }
        else {
            let chosen_media = MultiSelect::with_theme(&ColorfulTheme::default())
                .items(&media_resources)
                .with_prompt("Multiple media were found. Select the ones you want to download")
                .report(false)
                .interact()?;
            for i in chosen_media {
                plex_loader.download_media(&media_resources[i], &download_dir)
                    .with_context(|| "unable to download from given link")?;
            }
        }
        Ok(())
    }
}
