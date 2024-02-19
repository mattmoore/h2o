use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    #[command(about = "List available games.")]
    List,

    #[command(about = "Install game.")]
    Install { game: String },

    #[command(about = "Uninstall game.")]
    Uninstall { game: String },

    #[command(about = "Clear h2o download cache. No games will be uninstalled.")]
    ClearDownloads,

    #[command(about = "Play game.")]
    Play { game: String },
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub action: Commands,
}
