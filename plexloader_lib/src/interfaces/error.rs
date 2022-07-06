use thiserror::Error;
use url::{ParseError};

#[derive(Error, Debug)]
pub enum NetworkResponseError {
    #[error("unauthorized")]
    UnAuthorized,
    #[error("network error {0}")]
    UnknownNetworkError(#[from] ureq::Error),
    #[error("can't serialize to json {0}")]
    JSONSerializationError(#[from] serde_json::Error),
    #[error("can't serialize to xml {0}")]
    XMLSerializationError(#[from] quick_xml::de::DeError),
}

#[derive(Error, Debug)]
#[error("can't parse given link")]
pub enum MediaUriParsingError{
    ServerFetchError(#[from] ServerFetchError),
    URLParseError(#[from] ParseError),
    ServerNotFoundError(#[from] ServerNotFoundError),
}

#[derive(Error, Debug, PartialEq)]
#[error("can't find the given link in your accessible servers")]
pub struct ServerNotFoundError {}

#[derive(Error, Debug, PartialEq)]
#[error("can't find public uri of the server")]
pub struct URINotFoundError {}


#[derive(Error, Debug)]
#[error("can't fetch server details")]
pub enum ServerFetchError {
    URIError(#[from] URINotFoundError),
    NetworkError(#[from] NetworkResponseError)
}
