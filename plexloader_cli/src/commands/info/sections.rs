use clap::{Args};
use plexloader_lib::loader::PlexLoader;
use crate::utils::{info, success};

use plexloader_lib::PlexMediaSection;

use anyhow::{Result};

#[derive(Args)]
pub struct Sections {}

impl Sections {
    pub fn handle(self: &Self, plex_loader: PlexLoader) -> Result<()> {
        let servers = &plex_loader.servers;
        for server in servers {
            println!("Server: {}", success().apply_to(&server.name));
            let sections = plex_loader.get_sections(&server)?;
            for section in sections {
                match section { 
                    PlexMediaSection::MovieSection(s) => {
                        println!("\tMovie Section: {}", info().apply_to(s.title)); 
                    },
                    PlexMediaSection::TvSection(s) => {
                        println!("\tTv Section: {}", info().apply_to(s.title)); 
                    },
                }
            }
        }
        Ok(())
    }
}
