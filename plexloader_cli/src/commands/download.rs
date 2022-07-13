use clap::{Subcommand, Parser};

use crate::utils::{get_plex_loader};
use super::CommandHandler;

use anyhow::{Result};

mod link;
mod sections;
mod servers;

#[derive(Parser)]
pub struct Download {
    #[clap(subcommand)]
    command: DownloadCommands,
}

#[derive(Subcommand)]
enum DownloadCommands {
    /// Download from given link
    Link(link::Link),

    /// Download whole plex sections
    Sections(sections::Sections),

    /// Download whole plex servers
    Servers(servers::Servers),
}


impl CommandHandler for Download {
    fn handle(self: &Self) -> Result<()> {
        let plex_loader = get_plex_loader()?;
        match &self.command {
            DownloadCommands::Link(link) => link.handle(plex_loader)?,
            DownloadCommands::Sections(section) => section.handle(plex_loader)?,
            DownloadCommands::Servers(servers) => servers.handle(plex_loader)?,
        }
        Ok(())
    }
}
