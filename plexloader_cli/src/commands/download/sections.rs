use clap::{Args};
use plexloader_lib::loader::PlexLoader;
use crate::utils::{get_download_dir};
use plexloader_lib::loader::downloader::{
    download_section
};

use std::path::Path;

use plexloader_lib::{PlexServer, PlexMediaSection};

use dialoguer::{
    MultiSelect,
    Select,
    theme::ColorfulTheme
};

use anyhow::{Result};

fn download_sections(server: &PlexServer, plex_loader: &PlexLoader, download_dir: &Path) -> Result<()> {
    let sections = plex_loader.get_sections(server)?;
    let chosen_sections = MultiSelect::with_theme(&ColorfulTheme::default())
        .items(&sections)
        .with_prompt("Multiple sections were found. Select the ones you want to download")
        .report(false)
        .interact()?;
    for i in chosen_sections  {
        match &sections[i] {
            PlexMediaSection::MovieSection(s) => download_section(plex_loader, server , &s, download_dir)?,
            PlexMediaSection::TvSection(s) => download_section(plex_loader, server, &s, download_dir)?,
        };
    }
    Ok(())
}

#[derive(Args)]
pub struct Sections {}

impl Sections {
    pub fn handle(self: &Self, plex_loader: PlexLoader) -> Result<()> {
        let download_dir = get_download_dir();
        let servers = &plex_loader.servers;
        if servers.len() == 1 {
            download_sections(&servers[0], &plex_loader, &download_dir)?;
        }
        else {
            let chosen_server = Select::with_theme(&ColorfulTheme::default())
                .items(&servers)
                .with_prompt("Multiple servers were found. Select the one you want to download from")
                .report(false)
                .interact()?;
            download_sections(&servers[chosen_server], &plex_loader, &download_dir)?;
        }
        Ok(())
    }
}
