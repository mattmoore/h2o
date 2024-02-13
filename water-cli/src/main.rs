use clap::Parser;

use libwater::add;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    game: String,
}

fn main() {
    let args = Args::parse();

    println!("{}!", args.game);
    println!("Result: {}!", add(1, 2));
}
