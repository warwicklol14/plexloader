use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
#[serde(rename = "MediaContainer")]
pub struct PlexMediaContainer {
    #[serde(rename = "$value")]
    pub item: PlexContainerItem,
}

#[derive(Deserialize, Debug)]
pub enum PlexContainerItem {
    Video(PlexVideo),
    Directory(PlexDirectory)
}

#[derive(Deserialize, Debug)]
pub struct PlexDirectory {
    pub title: String,
    pub key: String,
}
    
#[derive(Deserialize, Debug)]
pub struct PlexVideo {
    #[serde(rename = "Media")]
    pub media: PlexMedia,

    pub title: String
}

#[derive(Deserialize, Debug)]
pub struct PlexMedia {
    #[serde(rename = "Part")]
    pub part: PlexMediaPart
}

#[derive(Deserialize, Debug)]
pub struct PlexMediaPart {
    pub key: String,
    pub file: String,
}

#[derive(Debug)]
pub struct PlexMediaMetadataUri {
    pub server_token: String,
    pub media_metadata_uri: String,
    pub server_uri: String
}


#[derive(Debug)]
pub struct PlexMediaResource {
    pub title: String,
    pub file_name: PathBuf,
    pub access_token: String,
    pub resource_path: String,
}

