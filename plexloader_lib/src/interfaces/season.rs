use serde::Deserialize;

use super::media::{PlexVideo};

#[derive(Deserialize, Debug)]
#[serde(rename = "MediaContainer")]
pub struct PlexSeasonContainer {
    #[serde(rename = "Video")]
    pub episodes: Vec<PlexVideo>,
}
