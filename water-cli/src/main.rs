use clap::Parser;

use libwater::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    game: String,
}

fn main() {
    let args = Args::parse();

    println!("{}!", args.game);
    let _ = process_file("/home/mattmoore/source/mattmoore/games/water/test_data/test_data.txt");
}
