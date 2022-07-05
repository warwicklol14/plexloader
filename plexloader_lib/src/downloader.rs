mod network;

use super::{PlexUser, LoginDetails, PlexServer};

#[derive(Debug)]
pub struct PlexDownloader {
    login_details: LoginDetails,
    plex_user: PlexUser
}

impl PlexDownloader {
    pub fn new(login_details: LoginDetails) -> PlexDownloader {
        PlexDownloader {
            plex_user: network::plex_login(&login_details.username, &login_details.password),
            login_details
        }
    }

    pub fn get_servers(&self) -> Vec<PlexServer> {
        network::plex_servers(&self.plex_user.auth_token).into_iter().filter(|server| server.access_token.is_some()).collect()
    }

}
