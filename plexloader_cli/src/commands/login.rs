use clap::{Args};

use plexloader_lib::downloader::login::plex_login_through_credentials;
use plexloader_lib::NetworkResponseError;

use crate::utils::serialize_plex_user;

#[derive(Args)]
pub struct Login {
    /// Plex username
    #[clap(short, long)]
    username: String,

    /// Plex password
    #[clap(short, long)]
    password: String,
}

impl Login {
    pub fn handle(self: &Self) {
        match plex_login_through_credentials(&self.username, &self.password) {
            Ok(plex_user) =>  {
                match serialize_plex_user(plex_user) {
                    Ok(_) => println!("Login was successfull! Saved token to file. Now you can use the download option"),
                    Err(e) => eprintln!("Error: Unable to write to auth file {:?}", e)
                }
            },
            Err(NetworkResponseError::UnAuthorized) => eprintln!("Error: Got unauthorized response. Please check your login credentials to see if they are correct!"),
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }
}
