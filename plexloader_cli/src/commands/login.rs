use clap::{Args};

use plexloader_lib::loader::login::plex_login_through_credentials;

use anyhow::{Context};

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

impl CommandHandler for Login {
    fn handle(self: &Self) -> anyhow::Result<()> {
        let plex_user = plex_login_through_credentials(&self.username, &self.password)
            .with_context(|| "unable to to login. check your credentials")?;
        serialize_plex_user(plex_user)
            .with_context(|| "unable to save auth info")?;
        println!("Login was successfull");
        Ok(())
    }
}
