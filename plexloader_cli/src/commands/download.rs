use clap::Args;

use crate::utils::deserialize_plex_user;
use crate::utils::PlexUserDeserializationError;
use plexloader_lib::downloader::PlexDownloader;

#[derive(Args)]
pub struct Download {
    /// Link for plex resource
    #[clap(short, long)]
    link: String,
}

impl Download {
    pub fn handle(self: &Self) {
        match deserialize_plex_user() {
            Ok(plex_user) => {
                let plex_downloader = PlexDownloader::new(plex_user);
                println!("{:?}", plex_downloader);
                println!("{:?}", plex_downloader.get_servers());
            }
            Err(e) => match e {
                PlexUserDeserializationError::FileError(e) => {
                    eprintln!("Unable to read auth.json file {:?}", e)
                }
                PlexUserDeserializationError::DeserializationError(e) => {
                    eprintln!("Unable to deserialize json file {:?}", e)
                }
            },
        }
    }
}
