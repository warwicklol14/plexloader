mod network;
pub mod login;

use super::{PlexUser, PlexServer};

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

    pub fn get_servers(&self) -> Vec<PlexServer> {
        network::plex_servers(&self.plex_user.auth_token).into_iter().filter(|server| server.access_token.is_some()).collect()
    }

}
