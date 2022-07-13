use clap::{Args};
use plexloader_lib::loader::PlexLoader;
use crate::utils::{get_download_dir};
use plexloader_lib::loader::downloader::{
    download_section
};
use plexloader_lib::utils::fs::create_dir;

use std::path::Path;

use plexloader_lib::{PlexServer, PlexMediaSection};

use dialoguer::{
    MultiSelect,
    theme::ColorfulTheme
};

use anyhow::{Result};


fn download_sections(server: &PlexServer, plex_loader: &PlexLoader, download_dir: &Path) -> Result<()> {
    let sections = plex_loader.get_sections(server)?;
    for section in &sections  {
        match section {
            PlexMediaSection::MovieSection(s) => download_section(plex_loader, server , &s, download_dir)?,
            PlexMediaSection::TvSection(s) => download_section(plex_loader, server, &s, download_dir)?,
        };
    }
    Ok(())
}

#[derive(Args)]
pub struct Servers {}

impl Servers {
    pub fn handle(self: &Self, plex_loader: PlexLoader) -> Result<()> {
        let mut download_dir = get_download_dir();
        let servers = &plex_loader.servers;
        if servers.len() == 1 {
            download_dir.push(&servers[0].name);
            create_dir(&download_dir)?;
            download_sections(&servers[0], &plex_loader, &download_dir)?;
        }
        let chosen_servers = MultiSelect::with_theme(&ColorfulTheme::default())
            .items(&servers)
            .with_prompt("Multiple servers were found. Select the ones you want to download from")
            .report(false)
            .interact()?;
        for i in chosen_servers {
            download_dir.push(&servers[i].name);
            create_dir(&download_dir)?;
            download_sections(&servers[i], &plex_loader, &download_dir)?;
        }
        Ok(())
    }
}
