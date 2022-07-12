use serde::Deserialize;
use crate::SectionConversionError;

#[derive(Deserialize, Debug)]
#[serde(rename = "MediaContainer")]
pub struct PlexSectionContainer {
    #[serde(rename = "MediaProvider")]
    pub provider: PlexSectionProvider,
}

#[derive(Deserialize, Debug)]
pub struct PlexSectionProvider {
    #[serde(rename = "Feature")]
    pub features: Vec<PlexSectionFeature>
}

#[derive(Deserialize, Debug, Clone)]
pub struct PlexSectionFeature {
    #[serde(rename = "Directory")]
    directories: Option<Vec<PlexFeatureDirectory>>
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum PlexFeatureDirectory {
    SectionDirectory(PlexSectionDirectory),
    HubDirectory(PlexHubDirectory)
}

#[derive(Deserialize, Debug, Clone)]
pub struct PlexHubDirectory {
    hubKey: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PlexSectionDirectory {
    key: String,
    title: String,
    
    #[serde(rename = "type")]
    directory_type: String,
}

#[derive(Deserialize, Debug)]
pub enum PlexMediaSection {
    MovieSection(PlexSection),
    TvSection(PlexSection)
}

#[derive(Deserialize, Debug)]
pub struct PlexSection {
    pub key: String,
    pub title: String,
}

// TODO : Refactor this ugly mess
impl TryFrom<PlexSectionContainer> for Vec<PlexMediaSection> {
    type Error = SectionConversionError;

    fn try_from(container: PlexSectionContainer) -> Result<Vec<PlexMediaSection>, SectionConversionError> {
        let mut sections: Vec<PlexMediaSection> = vec![];
        let features = container.provider.features;
        let directories_option = features[0].to_owned().directories;
        if let Some(directories) = directories_option {
            for directory in directories {
                match directory {
                    PlexFeatureDirectory::SectionDirectory(section) => {
                            if section.directory_type == "movie" {
                                sections.push(PlexMediaSection::MovieSection(PlexSection {
                                    key: section.key,
                                    title: section.title
                                }))
                            }
                            else if section.directory_type == "show" {
                                sections.push(PlexMediaSection::TvSection(PlexSection {
                                    key: section.key,
                                    title: section.title
                                }))
                            }
                    },
                    PlexFeatureDirectory::HubDirectory(_h) => continue,
                }
            }
        }
        Ok(sections)
    }
}
