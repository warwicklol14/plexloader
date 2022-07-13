mod network;
pub mod login;
pub mod downloader;
pub mod player;

use crate::interfaces::*;
use crate::utils::*;
use crate::constants::PLEX_EXCLUDE_ALLLEAVES;

fn construct_video_resource(access_token: &str, server_uri: &str , video: PlexVideo) -> Result<PlexMediaResource, MediaResourceFetchError>{
    let mut video_resources = vec![];
    for media in video.media {
        video_resources.push(PlexVideoResource {
            title: video.title.clone(),
            file_name: truncate_to_filename(media.part.file)?,
            access_token: String::from(access_token),
            resource_path: append_to_plex_server_uri(server_uri, &media.part.key),
        })
    }
    Ok(PlexMediaResource::VideoResource(video_resources))
}

fn construct_directory_children(server_uri: &str, episodes: Vec<PlexVideo>) -> Result<Vec<PlexDirectoryChild>, FileNameNotFoundError> {
    let mut directory_children = vec![];
    for episode in episodes {
        for media in episode.media {
            directory_children.push(PlexDirectoryChild {
                title: episode.title.clone(),
                file_name: truncate_to_filename(media.part.file)?,
                resource_path: append_to_plex_server_uri(server_uri, &media.part.key)
            })
        }
    }
    Ok(directory_children)
}

fn construct_season_resource(access_token: &str, server_uri: &str, directory: PlexDirectory) -> Result<PlexDirectoryResource, MediaResourceFetchError> {
    let link = append_to_plex_server_uri(&server_uri, &directory.key);
    let response = network::plex_media(&link, &access_token);
    let season = network::get_xml_from_response::<PlexSeasonContainer>(response)?;
    let children = construct_directory_children(&server_uri, season.episodes)?;
    Ok(PlexDirectoryResource {
        title: directory.title,
        access_token: String::from(access_token),
        children
    })
}

fn construct_directory_resource(access_token: &str, server_uri: &str, directory: PlexDirectory) -> Result<PlexMediaResource, MediaResourceFetchError> {
    let mut directory_resources = vec![];
    if directory.directory_type == "show" {
        let link = append_to_plex_server_uri(&server_uri, &directory.key);
        let link = append_to_plex_server_uri(&link, PLEX_EXCLUDE_ALLLEAVES);
        let response = network::plex_media(&link, &access_token);
        let show = network::get_xml_from_response::<PlexShowContainer>(response)?;
        for mut season in show.seasons {
            season.title = format!("{}/{}", directory.title, season.title);
            directory_resources.push(construct_season_resource(access_token, server_uri , season)?);
        }
    }
    else if directory.directory_type == "season" {
        directory_resources.push(construct_season_resource(access_token, server_uri , directory)?);
    }
    Ok(PlexMediaResource::DirectoryResource(directory_resources))
}

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

    pub fn get_media_resource(&self, media_link: &str) -> Result<PlexMediaResource, MediaResourceFetchError> {
        let req_media_metadata_uri = self.get_metadata_uri(media_link)?;
        let req_media_container = self.get_media(&req_media_metadata_uri)?;
        match req_media_container.item {
            PlexContainerItem::Video(v) => construct_video_resource(&req_media_metadata_uri.server_token, &req_media_metadata_uri.server_uri, v),
            PlexContainerItem::Directory(d) => construct_directory_resource(&req_media_metadata_uri.server_token, &req_media_metadata_uri.server_uri, d),
        }
    }

    pub fn get_section_items(&self, server: &PlexServer, section: &PlexSection) -> Result<Vec<PlexMediaResource>, MediaResourceFetchError> {
        let mut media_resources = vec![];
        let response = network::plex_section_items(&server.uri, &server.access_token, &section.key);
        let section_items = network::get_xml_from_response::<PlexSectionItemsContainer>(response)?;
        match section_items.items {
            PlexSectionItem::MovieSection(videos) => {
                for video in videos {
                    media_resources.push(construct_video_resource(&server.access_token, &server.uri, video)?);
                }
            },
            PlexSectionItem::TvSection(directories) => {
                for directory in directories {
                    media_resources.push(construct_directory_resource(&server.access_token, &server.uri, directory)?);
                }
            },
        }
        Ok(media_resources)
    }
}

