use dirs::home_dir;
use std::fs::File;
use std::io;
use std::process::Command;
use tar::Archive;
use xz::read::XzDecoder;

pub fn unpack_game(game: &str) -> io::Result<()> {
    if let Some(home) = home_dir() {
        let h2o_dir = &home.join(".h2o");
        let source = h2o_dir.join("downloads").join(format!("{game}-linux-x86_64.tar.xz"));
        let target = h2o_dir.join("games");

        println!("Installing {game}.");

        let tar_xz = File::open(source)?;
        let tar = XzDecoder::new(tar_xz);
        let _ = Archive::new(tar).unpack(target);

        println!("Finished installing {game}.");
    }

    Ok(())
}

pub fn run_game(game: &str) -> io::Result<()> {
    if let Some(home) = home_dir() {
        let h2o_dir = &home.join(".h2o");
        let target = h2o_dir
            .join("games")
            .join(game)
            .join("fantasy.sh");

        println!("target: {}", target.display());
        println!("Running {game}...");
        let _ = Command::new(target).output();
    }

    Ok(())
}
