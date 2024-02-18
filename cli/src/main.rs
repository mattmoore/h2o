use clap::{Parser, Subcommand};

use libh2o::*;

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    #[command(about = "Download a game.")]
    Download { game: String },
    #[command(about = "Run a game.")]
    Run { game: String },
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    action: Commands,
}

fn main() {
    let cmd = Args::parse();
    match &cmd.action {
        Commands::Download { game } => {
            unpack_game(&game.to_string()).expect("Failed to download {game}.")
        }
        Commands::Run { game } => {
            run_game(&game.to_string()).expect("Failed to run {game}.")
        }
    }
}
