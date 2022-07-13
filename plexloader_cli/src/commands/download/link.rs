use clap::{Args};
use plexloader_lib::loader::PlexLoader;
use plexloader_lib::PlexMediaResource;
use plexloader_lib::loader::downloader::download_aria;

use crate::utils::{get_download_dir, create_dir};

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
                    let mut aria_proc = download_aria(&videos[0].resource_path, &download_dir, &videos[0].file_name, &videos[0].access_token)
                        .with_context(|| "unable to spawn aria")?;
                    aria_proc.wait()
                        .with_context(|| "error occurred while downloading")?;
                }
                else {
                    let chosen_media = MultiSelect::with_theme(&ColorfulTheme::default())
                        .items(&videos)
                        .with_prompt("Multiple media were found. Select the ones you want to download")
                        .report(false)
                        .interact()?;
                    for i in chosen_media {
                        let mut aria_proc = download_aria(&videos[i].resource_path, &download_dir, &videos[i].file_name, &videos[i].access_token)
                            .with_context(|| "unable to spawn aria")?;
                        aria_proc.wait()
                            .with_context(|| "error occurred while downloading")?;
                    }
                }
            },
            PlexMediaResource::DirectoryResource(directories) => {
                for directory in directories {
                    if directory.title.contains("/") {
                        let paths = directory.title.split("/").collect::<Vec<&str>>();
                        let mut download_dir = download_dir.clone();
                        download_dir.push(paths[0]);
                        download_dir.push(paths[1]);
                        create_dir(&download_dir)?;
                        for child in directory.children {
                            let mut aria_proc = download_aria(&child.resource_path, &download_dir, &child.file_name, &directory.access_token)
                                .with_context(|| "unable to spawn aria")?;
                            aria_proc.wait()
                                .with_context(|| "error occurred while downloading")?;
                        }
                    }
                    else {
                        let mut download_dir = download_dir.clone();
                        download_dir.push(directory.title);
                        create_dir(&download_dir)?;
                        for child in directory.children {
                            let mut aria_proc = download_aria(&child.resource_path, &download_dir, &child.file_name, &directory.access_token)
                                .with_context(|| "unable to spawn aria")?;
                            aria_proc.wait()
                                .with_context(|| "error occurred while downloading")?;
                        }
                    }
                }
            },
        }
        Ok(())
    }
}
