use clap::{Subcommand, Parser};

use crate::utils::{get_plex_loader};
use super::CommandHandler;

use anyhow::{Result};

mod link;

#[derive(Parser)]
pub struct Download {
    #[clap(subcommand)]
    command: DownloadCommands,
}

#[derive(Subcommand)]
enum DownloadCommands {
    /// Download from given link
    Link(link::Link),
}


impl CommandHandler for Download {
    fn handle(self: &Self) -> Result<()> {
        let plex_loader = get_plex_loader()?;
        match &self.command {
            DownloadCommands::Link(link) => link.handle(plex_loader)?,
        }
        Ok(())
    }
}
