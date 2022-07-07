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
    Download(commands::Download),

    /// Information about requested resource from Plex
    Info(commands::Info),
}

fn main() {
    utils::print_err(utils::init_dirs());

    let cli = PlexLoaderCli::parse();

    match &cli.command {
        Commands::Login(login) => commands::handle_command(login),
        Commands::Download(download) => commands::handle_command(download),
        Commands::Info(info) => commands::handle_command(info),
    };
}
