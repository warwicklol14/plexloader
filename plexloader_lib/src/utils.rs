use crate::{Connections, PlexServer, Resource, ServerNotFoundError, URINotFoundError, FileNameNotFoundError};
use url::{ParseError, Url};
use std::path::{Path};
use crate::constants::PLEX_TOKEN_HEADER_NAME;

pub mod fs;

pub fn construct_token_header(server_token: &str) -> String {
    format!("{}: {}", PLEX_TOKEN_HEADER_NAME, server_token)
}

pub fn truncate_to_filename(mut media_file_name_path: String) -> Result<String, FileNameNotFoundError> {
    let filename = Path::new(&media_file_name_path)
        .file_name().ok_or(FileNameNotFoundError {})?;
    
    let filename_len = filename.to_str()
        .ok_or(FileNameNotFoundError {})?
        .len();
    
    media_file_name_path.drain(..media_file_name_path.len() - filename_len);
    
    Ok(media_file_name_path)
}

pub fn get_media_metadata_from_url(url: &str) -> Result<(String, String), ParseError> {
    let parsed_url = Url::parse(url)?;
    match parsed_url.fragment() {
        Some(server_fragment) => {
            let server_url = parsed_url.join(server_fragment)?;
            let metadata_key = server_url
                .query_pairs()
                .next()
                .ok_or_else(|| ParseError::InvalidIpv4Address)?
                .1
                .to_string();
            let server_hash = server_url
                .path_segments()
                .ok_or_else(|| ParseError::InvalidIpv4Address)?
                .nth(3)
                .ok_or_else(|| ParseError::InvalidIpv4Address)?
                .to_string();
            Ok((server_hash, metadata_key))
        }
        None => Err(ParseError::InvalidIpv4Address),
    }
}

pub fn find_server_from_hash<'a>(
    servers: &'a Vec<PlexServer>,
    server_hash: &str,
) -> Result<&'a PlexServer, ServerNotFoundError> {
    servers
        .iter()
        .find(|s| s.client_identifier.eq(server_hash))
        .ok_or(ServerNotFoundError {})
}

pub fn map_resource_to_server(
    resources: Vec<Resource>,
) -> Result<Vec<PlexServer>, URINotFoundError> {
    resources.into_iter().map(|r| r.try_into()).collect()
}

pub fn filter_non_servers(resources: Vec<Resource>) -> Vec<Resource> {
    resources
        .into_iter()
        .filter(|r| !r.access_token.is_empty())
        .collect()
}

pub fn find_public_uri_from_connections(
    connections: Connections,
    public_address: String,
) -> Result<String, URINotFoundError> {
    let domain_connection = connections
        .connection
        .iter()
        .find(|c| c.port == 443);
    match domain_connection {
        Some(c) => Ok(c.uri.clone()),
        None => {
            connections
                .connection
                .into_iter()
                .find(|c| c.address == public_address)
                .ok_or(URINotFoundError {})
                .map(|c| c.uri)
        }
    }
}

pub fn append_to_plex_server_uri(plex_server_uri: &str, append_str: &str) -> String {
    format!("{}{}", plex_server_uri, append_str)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Connection, Connections, PlexServer};

    #[test]
    fn metadata_parsing_from_url_works() {
        let test_url = "https://app.plex.tv/desktop/#!/server/testhash/details?key=%2Flibrary%2Fmetadata%2F627581";
        assert_eq!(
            get_media_metadata_from_url(test_url),
            Ok((
                "testhash".to_string(),
                "/library/metadata/627581".to_string()
            ))
        );
    }

    #[test]
    fn can_filter_non_servers() {
        let resources = vec![
            Resource {
                name: "Android-Phone".to_string(),
                client_identifier: "hash-com-plexapp-android".to_string(),
                access_token: "".to_string(),
                public_address: "8.8.8.8".to_string(),
                connections: Connections {
                    connection: vec![Connection {
                        uri: "http://192.168.0.1:32500".to_string(),
                        port: 32500,
                        address: "192.168.0.1".to_string(),
                    }],
                },
            },
            Resource {
                name: "Plex Server".to_string(),
                client_identifier: "hash-com-plexapp-android".to_string(),
                access_token: "token".to_string(),
                public_address: "plex.tv".to_string(),
                connections: Connections {
                    connection: vec![Connection {
                        uri: "http://plex.tv:443".to_string(),
                        port: 443,
                        address: "plex.tv".to_string(),
                    }],
                },
            },
        ];
        assert_eq!(
            filter_non_servers(resources.clone()),
            vec![resources[1].clone()]
        );
    }

    #[test]
    fn can_get_file_uri() {
        let test_plex_server_uri = "https://plex.tv:443";
        let file_key = "/library/parts/890344/1598858660/file.mkv";
        assert_eq!(
            append_to_plex_server_uri(test_plex_server_uri, file_key),
            "https://plex.tv:443/library/parts/890344/1598858660/file.mkv"
        );
    }

    #[test]
    fn can_get_metadata_url() {
        let test_plex_server_uri = "https://plex.tv:443";
        let test_metadata_key_path = "/library/metadata/627581";
        assert_eq!(
            append_to_plex_server_uri(test_plex_server_uri, test_metadata_key_path),
            "https://plex.tv:443/library/metadata/627581"
        );
    }

    #[test]
    fn can_find_server_from_hash() {
        let test_plex_servers = vec![
            PlexServer {
                name: "Test Server".to_string(),
                access_token: "test_token".to_string(),
                client_identifier: "test identifier".to_string(),
                uri: "test uri".to_string(),
            },
            PlexServer {
                name: "Test Server".to_string(),
                access_token: "test_token".to_string(),
                client_identifier: "test identifier 2".to_string(),
                uri: "test uri".to_string(),
            },
        ];

        assert_eq!(
            find_server_from_hash(&test_plex_servers, "test identifier"),
            Ok(&test_plex_servers[0])
        );
    }
    #[test]
    fn can_find_public_uri_from_connections() {
        let test_plex_server = Connections {
            connection: vec![
                Connection {
                    port: 32400,
                    uri: "https://192-168-0-1.plex.direct:32400".to_string(),
                    address: "192.168.0.1".to_string(),
                },
                Connection {
                    port: 32400,
                    uri: "https://8-8-8-8.hash.plex.direct:32400".to_string(),
                    address: "8.8.8.8".to_string(),
                },
            ],
        };

        assert_eq!(
            find_public_uri_from_connections(test_plex_server, "8.8.8.8".to_string()),
            Ok("https://8-8-8-8.hash.plex.direct:32400".to_string()),
        );
    }

    fn can_find_domain_uri_from_connections() {
        let test_plex_server = Connections {
            connection: vec![
                Connection {
                    port: 443,
                    uri: "https://plex.tv:443".to_string(),
                    address: "plex.tv".to_string(),
                },
                Connection {
                    port: 32400,
                    uri: "https://192-168-0-1.plex.direct:32400".to_string(),
                    address: "192.168.0.1".to_string(),
                },
                Connection {
                    port: 32400,
                    uri: "https://8-8-8-8.hash.plex.direct:32400".to_string(),
                    address: "8.8.8.8".to_string(),
                },
            ],
        };

        assert_eq!(
            find_public_uri_from_connections(test_plex_server, "8.8.8.8".to_string()),
            Ok("https://plex.tv:443".to_string()),
        );
    }
}
