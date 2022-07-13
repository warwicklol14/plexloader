use serde::Deserialize;
use super::media::PlexDirectory;

#[derive(Deserialize, Debug)]
#[serde(rename = "MediaContainer")]
pub struct PlexShowContainer {
    #[serde(rename = "Directory")]
    pub seasons: Vec<PlexDirectory>,
}
