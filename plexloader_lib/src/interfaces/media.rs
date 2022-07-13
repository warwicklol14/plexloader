use serde::Deserialize;
use std::fmt;

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

    #[serde(rename = "type")]
    pub directory_type: String,
}
    
#[derive(Deserialize, Debug)]
pub struct PlexVideo {
    #[serde(rename = "Media")]
    pub media: Vec<PlexMedia>,

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
pub enum PlexMediaResource {
    VideoResource(Vec<PlexVideoResource>),
    DirectoryResource(Vec<PlexDirectoryResource>)
}

#[derive(Debug)]
pub struct PlexDirectoryResource {
    pub title: String,
    pub access_token: String,
    pub children: Vec<PlexDirectoryChild>,
}

#[derive(Debug)]
pub struct PlexDirectoryChild {
    pub title: String,
    pub file_name: String,
    pub resource_path: String,
}

#[derive(Debug)]
pub struct PlexVideoResource {
    pub title: String,
    pub file_name: String,
    pub access_token: String,
    pub resource_path: String,
}

impl fmt::Display for PlexVideoResource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\t{}", self.title, self.file_name)
    }
}

impl fmt::Display for PlexDirectoryResource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n", self.title)?;
        for child in &self.children {
            write!(f, "\t{}\t{}\n", child.title, child.file_name)?;
        }
        Ok(())
    }
}
