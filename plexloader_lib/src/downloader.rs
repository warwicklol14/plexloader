mod network;
pub mod login;

use super::{PlexUser, PlexServer, PlexMediaMetadataUri, PlexMediaContainer, Resources};
use crate::{NetworkResponseError, MediaUriParsingError, ServerFetchError};
use crate::utils::*;

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

    fn get_servers(&self) -> Result<Vec<PlexServer>, ServerFetchError> {
        let resources = self.get_resources()?;
        let resources = filter_non_servers(resources.resource); 
        let servers = map_resource_to_server(resources)?;
        Ok(servers)
    }

    fn get_resources(&self) -> Result<Resources, NetworkResponseError> {
        let response = network::plex_resources(&self.plex_user.auth_token);
        let plex_resources = network::get_xml_from_response::<Resources>(response)?;
        Ok(plex_resources)
    }

    pub fn get_media(&self, plex_media_metadata_uri: &PlexMediaMetadataUri) -> Result<PlexMediaContainer, NetworkResponseError> {
        let response = network::plex_media(&plex_media_metadata_uri.media_metadata_uri, &plex_media_metadata_uri.server_token);
        network::get_xml_from_response::<PlexMediaContainer>(response)
    }

    pub fn get_metadata_uri(&self, media_link: &str) -> Result<PlexMediaMetadataUri, MediaUriParsingError> {
        let servers = self.get_servers()?;
        let (server_hash, media_key) = get_media_metadata_from_url(media_link)?;
        let req_server = find_server_from_hash(servers, &server_hash)?;
        let metadata_uri = append_to_plex_server_uri(&req_server.uri, &media_key);
        Ok(PlexMediaMetadataUri {
            server_token: req_server.access_token,
            media_metadata_uri: metadata_uri,
        })
    }

}
