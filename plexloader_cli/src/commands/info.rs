use clap::{Parser, Subcommand};

use crate::utils::{get_plex_loader};
use super::CommandHandler;

#[derive(Parser)]
pub struct Info {
    #[clap(subcommand)]
    command: InfoCommands,
}

mod link;
mod servers;

#[derive(Subcommand)]
enum InfoCommands {
    /// Prints information about given link
    Link(link::Link),

    /// Prints information about the servers you are in
    Servers(servers::Servers),
}

impl CommandHandler for Info {
    fn handle(self: &Self) -> anyhow::Result<()> {
        let plex_loader = get_plex_loader()?;
        match &self.command {
            InfoCommands::Link(link) => link.handle(plex_loader)?,
            InfoCommands::Servers(servers) => servers.handle(plex_loader)?,
        }
        Ok(())
    }
}

