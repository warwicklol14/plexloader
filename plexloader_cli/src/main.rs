use clap::{Parser, Subcommand};

mod commands;
mod utils;

#[derive(Parser)]
#[clap(version)]
struct PlexLoaderCli {
    #[clap(subcommand)]
    command: Commands,
}


#[derive(Subcommand)]
enum Commands {
    /// Login to Plex
    Login(commands::Login),

    /// Download from Plex
    Download(commands::Download)
}

fn main() {
    let cli = PlexLoaderCli::parse();

    match &cli.command {
        Commands::Login(login) => login.handle(),
        Commands::Download(download) => download.handle(),
    }
}
    
