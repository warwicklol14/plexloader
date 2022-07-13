use clap::{Args};

use plexloader_lib::loader::login::plex_login_through_credentials;

use dialoguer::{
    Input,
    theme::ColorfulTheme,
};

use anyhow::{Context};

use crate::utils::{serialize_plex_user, success};
use super::CommandHandler;

#[derive(Args)]
pub struct Login {}

impl CommandHandler for Login {
    fn handle(self: &Self) -> anyhow::Result<()> {
        let username: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Your plex username")
            .interact()?;
        let password: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Your plex password")
            .interact()?;
        let plex_user = plex_login_through_credentials(&username, &password)
            .with_context(|| "unable to to login. check your credentials")?;
        serialize_plex_user(plex_user)
            .with_context(|| "unable to save auth info")?;
        println!("{}", success().apply_to("Login was successfull"));
        Ok(())
    }
}
