use super::network; 
use crate::{PlexUserResponse, PlexUser, NetworkResponseError};

pub fn plex_login_through_credentials(username: &str, password: &str) -> Result<PlexUser, NetworkResponseError> {
    let response = network::plex_login(username, password);
    Ok(network::get_json_from_response::<PlexUserResponse>(response)?.user)
}
