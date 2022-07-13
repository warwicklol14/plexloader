use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Resources {
    pub resource: Vec<Resource>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Resource {
    pub name: String,

    #[serde(rename = "clientIdentifier")]
    pub client_identifier: String,

    #[serde(rename = "publicAddress")]
    pub public_address: String,

    #[serde(rename = "accessToken")]
    pub access_token: String,

    pub connections: Connections,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Connections{
    pub connection: Vec<Connection>,
}


#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Connection {
    pub uri: String,
    pub port: usize,
    pub address: String,
}
