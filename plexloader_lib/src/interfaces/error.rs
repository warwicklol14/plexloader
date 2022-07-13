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

#[derive(Error, Debug)]
#[error("can't find file name")]
pub struct FileNameNotFoundError {}

#[derive(Error, Debug, PartialEq)]
#[error("can't find the given link in your accessible servers")]
pub struct ServerNotFoundError {}

#[derive(Error, Debug, PartialEq)]
#[error("can't find public uri of the server")]
pub struct URINotFoundError {}

#[derive(Error, Debug)]
#[error("can't convert sections from deserialized values")]
pub struct SectionConversionError {}

#[derive(Error, Debug)]
#[error("can't fetch section details")]
pub enum SectionFetchError {
    NetworkError(#[from] NetworkResponseError),
    ConversionError(#[from] SectionConversionError),
}

#[derive(Error, Debug)]
#[error("can't fetch server details")]
pub enum ServerFetchError {
    URIError(#[from] URINotFoundError),
    NetworkError(#[from] NetworkResponseError)
}

#[derive(Error, Debug)]
#[error("can't fetch plex media resource details")]
pub enum MediaResourceFetchError {
    ParsingError(#[from] MediaUriParsingError),
    NetworkError(#[from] NetworkResponseError),
    FileNameError(#[from] FileNameNotFoundError),
}

#[derive(Error, Debug)]
#[error("can't download media")]
pub enum MediaDownloadError {
    AriaError(#[from] std::io::Error),
    DownloadDirectoryCreationError(#[from] DirectoryCreationError),
}

#[derive(Error, Debug)]
#[error("can't download section")]
pub enum SectionDownloadError {
    MediaDownloadError(#[from] MediaDownloadError),
    SectionFetchError(#[from] MediaResourceFetchError),
}

#[derive(Error, Debug)]
#[error("can't playback media")]
pub enum MediaPlaybackError {
    MpvError(#[from] std::io::Error),
}

#[derive(Error, Debug)]
#[error("can't create directory")]
pub enum DirectoryCreationError {
    CreateDirError(#[from] std::io::Error),
}

