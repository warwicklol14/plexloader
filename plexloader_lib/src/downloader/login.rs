use super::network; 
use crate::PlexUser;

pub fn plex_login_through_credentials(username: &str, password: &str) -> PlexUser {
    network::plex_login(username, password)
}
