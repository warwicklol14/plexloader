use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename = "MediaContainer")]
pub struct PlexMediaContainer {
    #[serde(rename = "Metadata")]
    metadata: Vec<PlexMediaMetadata>,
}

#[derive(Deserialize, Debug)]
pub struct PlexMediaMetadata {
    key: String,

    #[serde(rename = "type")]
    media_type: String,

    #[serde(rename = "Media")]
    media: PlexMedia
}

#[derive(Deserialize, Debug)]
pub struct PlexMedia {
    #[serde(rename = "Part")]
    part: PlexMediaPart
}

#[derive(Deserialize, Debug)]
pub struct PlexMediaPart {
    key: String,
}

