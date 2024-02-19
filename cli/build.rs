use clap::{CommandFactory, ValueEnum};
use clap_complete::{generate_to, Shell};
use std::env;
use std::io::Error;

include!("src/cli.rs");

fn main() -> Result<(), Error> {
    let out_dir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let mut cmd = Args::command();

    for &shell in Shell::value_variants() {
        let path = generate_to(shell, &mut cmd, "h2o", &out_dir);
        println!("cargo:warning=completion file for {shell} is generated: {path:?}");
    }

    Ok(())
}
