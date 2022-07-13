use clap::{Args};
use plexloader_lib::loader::PlexLoader;
use crate::utils::{info, success};

use anyhow::{Result};

#[derive(Args)]
pub struct Servers {}

impl Servers {
    pub fn handle(self: &Self, plex_loader: PlexLoader) -> Result<()> {
        let servers = plex_loader.servers;
        println!("{}", success().apply_to("These are the following servers you are in:"));
        for server in servers {
            println!("\t{}", info().apply_to(&server));
        }
        Ok(())
    }
}
