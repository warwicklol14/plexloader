use clap::{Args};

use plexloader_lib::loader::login::plex_login_through_credentials;
use plexloader_lib::NetworkResponseError;

use crate::utils::serialize_plex_user;
use super::CommandHandler;

#[derive(Args)]
pub struct Login {
    /// Plex username
    #[clap(short, long)]
    username: String,

    /// Plex password
    #[clap(short, long)]
    password: String,
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoginCommandError{
    #[error("Error: net {0}")]
    NetworkResponseError(#[from] NetworkResponseError),
}

impl CommandHandler for Login {
    fn handle(self: &Self) -> anyhow::Result<()> {
        let plex_user = plex_login_through_credentials(&self.username, &self.password)?;
        serialize_plex_user(plex_user)?;
        println!("Login was successfull");
        Ok(())
    }
}
