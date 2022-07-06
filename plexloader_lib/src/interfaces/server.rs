use crate::utils::find_public_uri_from_connections;
use crate::Resource;
use crate::URINotFoundError;

#[derive(Debug, PartialEq, Clone)]
pub struct PlexServer {
    pub name: String,
    pub access_token: String,
    pub client_identifier: String,
    pub uri: String,
}

impl TryFrom<Resource> for PlexServer {
    type Error = URINotFoundError;

    fn try_from(resource: Resource) -> Result<Self, Self::Error> {
        find_public_uri_from_connections(resource.connections).map(|uri| PlexServer {
            name: resource.name,
            client_identifier: resource.client_identifier,
            access_token: resource.access_token,
            uri,
        })
    }
}
