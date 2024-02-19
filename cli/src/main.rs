mod cli;

use clap::{CommandFactory, Parser};
use cli::*;
use h2o::*;

fn main() {
    let _m = Args::command().get_matches();

    match Args::parse().action {
        Commands::List => list().expect("Failed to list catalog."),
        Commands::Install { game } => install(&game).expect("Failed to install {game}."),
        Commands::Uninstall { game } => uninstall(&game).expect("Failed to install {game}."),
        Commands::ClearDownloads => clear_downloads().expect("Failed to clear the download cache."),
        Commands::Play { game } => play(&game).expect("Failed to run {game}."),
    }
}
