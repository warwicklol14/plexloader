mod network;
pub mod login;

use super::{PlexUser, PlexServer, PlexMediaMetadataUri, PlexMediaContainer, Resources, PlexMediaResource, PlexContainerItem};
use crate::{NetworkResponseError, MediaUriParsingError, ServerFetchError, MediaResourceFetchError};
use crate::utils::*;

fn get_resources(plex_user: &PlexUser) -> Result<Resources, NetworkResponseError> {
    let response = network::plex_resources(&plex_user.auth_token);
    let plex_resources = network::get_xml_from_response::<Resources>(response)?;
    Ok(plex_resources)
}

pub fn get_servers(plex_user: &PlexUser) -> Result<Vec<PlexServer>, ServerFetchError> {
    let resources = get_resources(plex_user)?;
    let resources = filter_non_servers(resources.resource); 
    let servers = map_resource_to_server(resources)?;
    Ok(servers)
}

#[derive(Debug)]
pub struct PlexLoader {
    plex_user: PlexUser,
    servers: Vec<PlexServer>
}

impl PlexLoader {
    pub fn new(plex_user: PlexUser) -> Result<PlexLoader, ServerFetchError> {
        Ok(
            PlexLoader {
                servers: get_servers(&plex_user)?,
                plex_user,
            }
        )
    }

    fn get_media(&self, plex_media_metadata_uri: &PlexMediaMetadataUri) -> Result<PlexMediaContainer, NetworkResponseError> {
        let response = network::plex_media(&plex_media_metadata_uri.media_metadata_uri, &plex_media_metadata_uri.server_token);
        network::get_xml_from_response::<PlexMediaContainer>(response)
    }

    fn get_metadata_uri(&self, media_link: &str) -> Result<PlexMediaMetadataUri, MediaUriParsingError> {
        let (server_hash, media_key) = get_media_metadata_from_url(media_link)?;
        let req_server = find_server_from_hash(&self.servers, &server_hash)?;
        let metadata_uri = append_to_plex_server_uri(&req_server.uri, &media_key);
        Ok(PlexMediaMetadataUri {
            server_token: req_server.access_token.clone(),
            media_metadata_uri: metadata_uri,
            server_uri: req_server.uri.clone()
        })
    }

    pub fn get_media_resource(&self, media_link: &str) -> Result<PlexMediaResource, MediaResourceFetchError> {
        let req_media_metadata_uri = self.get_metadata_uri(media_link)?;
        let req_media_container = self.get_media(&req_media_metadata_uri)?;
        match req_media_container.item {
            PlexContainerItem::Video(v) => {
                Ok(PlexMediaResource {
                    name: v.title,
                    resource_path: append_to_plex_server_uri(&req_media_metadata_uri.server_uri, &v.media.part.key),
                    children: None
                })
            }
            PlexContainerItem::Directory(d) => {
                Ok(PlexMediaResource {
                    name: d.title,
                    resource_path: d.key,
                    children: None
                })
            }
        }
    }

}
