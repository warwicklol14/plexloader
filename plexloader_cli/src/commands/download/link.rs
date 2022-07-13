use clap::{Args};
use plexloader_lib::loader::PlexLoader;
use plexloader_lib::loader::downloader::{
    download_video_resource,
    download_directory_resource
};
use plexloader_lib::{PlexMediaResource};

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
        let media_resource = plex_loader.get_media_resource(&self.link)
            .with_context(|| "unable to get media")?;
        match media_resource {
            PlexMediaResource::VideoResource(videos) => {
                if videos.len() == 1 {
                    download_video_resource(&videos[0], &download_dir)?;
                }
                else {
                    let chosen_media = MultiSelect::with_theme(&ColorfulTheme::default())
                        .items(&videos)
                        .with_prompt("Multiple media were found. Select the ones you want to download")
                        .report(false)
                        .interact()?;
                    for i in chosen_media {
                        download_video_resource(&videos[i], &download_dir)?;
                    }
                }
            },
            PlexMediaResource::DirectoryResource(directories) => {
                for directory in directories {
                    download_directory_resource(&directory, &download_dir)?;
                }
            },
        }
        Ok(())
    }
}
