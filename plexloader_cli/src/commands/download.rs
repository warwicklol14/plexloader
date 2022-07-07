use clap::{Subcommand, Parser};

use crate::utils::{deserialize_plex_user};
use plexloader_lib::loader::PlexLoader;
use super::CommandHandler;

use anyhow::{Result, Context};

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
        let plex_user = deserialize_plex_user()
            .with_context(|| "unable to use previous auth. try logging in again?")?;
        let plex_loader = PlexLoader::new(plex_user);
        match &self.command {
            DownloadCommands::Link(link) => link.handle(plex_loader)?,
        }
        Ok(())
    }
}
