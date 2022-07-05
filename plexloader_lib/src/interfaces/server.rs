use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PlexServer {
    pub name: String,
    pub product: String,
    pub provides: String,

    #[serde(rename = "accessToken")]
    pub access_token: Option<String>,

    pub connections: Vec<PlexServerConnection>,
}

#[derive(Deserialize, Debug)]
pub struct PlexServerConnection {
    pub address: String,
    pub uri: String
}
