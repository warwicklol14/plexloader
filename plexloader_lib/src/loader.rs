mod network;
pub mod login;
mod downloader;
mod player;

use crate::interfaces::*;
use crate::utils::*;
use downloader::download_aria;
use player::play_media_mpv;
use std::path::{Path};


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

pub struct PlexLoader {
    plex_user: PlexUser,
    pub servers: Vec<PlexServer>
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

    pub fn get_sections(&self, server: &PlexServer) -> Result<Vec<PlexMediaSection>, SectionFetchError> {
        let req_server = self.servers.iter().find(|&s| s == server).unwrap();
        let response = network::plex_sections(&req_server.uri, &req_server.access_token);
        let container = network::get_xml_from_response::<PlexSectionContainer>(response)?;
        let sections = container.try_into()?;
        Ok(sections)
    }

    pub fn get_media_resources(&self, media_link: &str) -> Result<Vec<PlexMediaResource>, MediaResourceFetchError> {
        let req_media_metadata_uri = self.get_metadata_uri(media_link)?;
        let req_media_container = self.get_media(&req_media_metadata_uri)?;
        match req_media_container.item {
            PlexContainerItem::Video(v) => {
                let mut media_resources = vec![];
                for media in v.media {
                    media_resources.push(PlexMediaResource {
                        title: v.title.clone(),
                        file_name: truncate_to_filename(media.part.file)?,
                        access_token: req_media_metadata_uri.server_token.clone(),
                        resource_path: append_to_plex_server_uri(&req_media_metadata_uri.server_uri, &media.part.key),
                    })
                }
                Ok(media_resources)
            }
            PlexContainerItem::Directory(d) => {
                Ok(vec![(PlexMediaResource {
                    title: d.title,
                    file_name: String::new(),
                    access_token: req_media_metadata_uri.server_token,
                    resource_path: d.key,
                })])
            }
        }
    }

    pub fn download_media(&self, media_resource: &PlexMediaResource, download_dir_path: &Path) -> Result<(), MediaDownloadError>{ 
        let mut child = download_aria(&media_resource.resource_path, &download_dir_path , &media_resource.file_name, &media_resource.access_token)?;
        child.wait()?;
        Ok(())
    }

    pub fn playback_media(&self, media_resource: &PlexMediaResource) -> Result<(), MediaPlaybackError>{ 
        let mut child = play_media_mpv(&media_resource.resource_path, &media_resource.title, &media_resource.access_token)?;
        child.wait()?;
        Ok(())
    }
}
