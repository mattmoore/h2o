use clap::{Parser, Subcommand};

use libh2o::*;

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    #[command(about = "List available games.")]
    List,
    #[command(about = "Install a game.")]
    Install { game: String },
    #[command(about = "Uninstall a game.")]
    Uninstall { game: String },
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
        Commands::List => {
            list().expect("Failed to list catalog.")
        }
        Commands::Install { game } => {
            install(game.to_string()).expect("Failed to install {game}.")
        }
        Commands::Uninstall { game } => {
            uninstall(game.to_string()).expect("Failed to install {game}.")
        }
        Commands::Run { game } => {
            run(game.to_string()).expect("Failed to run {game}.")
        }
    }
}
