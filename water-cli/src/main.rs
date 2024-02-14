use clap::Parser;

use libwater::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    game: String,

    #[arg(short, long)]
    source: String,

    #[arg(short, long)]
    target: String,
}

fn main() {
    let args = Args::parse();

    println!("{}!", args.game);
    let _ = decompress(args.source, args.target);
}
