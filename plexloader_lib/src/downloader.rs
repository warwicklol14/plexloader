mod network;
pub mod login;

use super::{PlexUser, PlexServer};
use crate::NetworkResponseError;

#[derive(Debug)]
pub struct PlexDownloader {
    plex_user: PlexUser
}

impl PlexDownloader {
    pub fn new(plex_user: PlexUser) -> PlexDownloader {
        PlexDownloader {
            plex_user
        }
    }

    pub fn get_servers(&self) -> Result<Vec<PlexServer>, NetworkResponseError> {
        let response = network::plex_servers(&self.plex_user.auth_token);
        let plex_servers = network::get_json_from_response::<Vec<PlexServer>>(response)?;
        Ok(plex_servers
            .into_iter()
            .filter(|server| server.access_token.is_some())
            .collect())
    }

}
