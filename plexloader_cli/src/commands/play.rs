use anyhow::Context;
use clap::Args;
use dialoguer::{theme::ColorfulTheme, Select};
use plexloader_lib::loader::player::play_media_mpv;
use plexloader_lib::PlexMediaResource;

use super::CommandHandler;
use crate::utils::get_plex_loader;

#[derive(Args)]
pub struct Play {
    /// Plex resource link
    link: String,
}

impl CommandHandler for Play {
    fn handle(self: &Self) -> anyhow::Result<()> {
        let plex_loader = get_plex_loader()?;
        let media_resource = plex_loader
            .get_media_resource(&self.link)
            .with_context(|| "unable to get media")?;
        match media_resource {
            PlexMediaResource::VideoResource(videos) => {
                if videos.len() == 1 {
                    let mut child = play_media_mpv(
                        &videos[0].resource_path,
                        &videos[0].title,
                        &videos[0].access_token,
                    )
                    .with_context(|| "unable to spawn mpv")?;
                    child.wait().with_context(|| "error while playing link")?;
                } else {
                    let chosen_media = Select::with_theme(&ColorfulTheme::default())
                        .items(&videos)
                        .with_prompt("Multiple media were found. Select the one you want to play")
                        .report(false)
                        .interact()?;
                    let mut child = play_media_mpv(
                        &videos[chosen_media].resource_path,
                        &videos[chosen_media].title,
                        &videos[chosen_media].access_token,
                    )
                    .with_context(|| "unable to spawn mpv")?;
                    child.wait().with_context(|| "error while playing link")?;
                }
            }
            PlexMediaResource::DirectoryResource(_directories) => {}
        }
        Ok(())
    }
}
