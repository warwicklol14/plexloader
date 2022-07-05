use plexloader_lib::LoginDetails;
use plexloader_lib::downloader::{PlexDownloader};

use clap::Parser;

#[derive(Parser)]
#[clap(version)]
struct PlexLoaderCli {
    /// Plex username
    #[clap(short, long)]
    username: String,

    /// Plex password
    #[clap(short, long)]
    password: String,
}

fn main() {
    let args = PlexLoaderCli::parse();

    let user_login = LoginDetails {
        username: args.username,
        password: args.password
    };

    let plex_downloader = PlexDownloader::new(user_login);

    println!("{:?}", plex_downloader);

    println!("{:?}", plex_downloader.get_servers());
}
    
