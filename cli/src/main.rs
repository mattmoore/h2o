mod cli;

use clap::{CommandFactory, Parser};
use cli::*;
use h2o::*;

fn main() {
    let _m = Args::command().get_matches();

    match Args::parse().action {
        Commands::List => list().expect("Failed to list catalog."),
        Commands::Download { game } => download(&game).expect("Failed to install {game}."),
        Commands::Install { game } => install(&game).expect("Failed to install {game}."),
        Commands::Uninstall { game } => uninstall(&game).expect("Failed to install {game}."),
        Commands::Play { game } => play(&game).expect("Failed to run {game}."),
    }
}
