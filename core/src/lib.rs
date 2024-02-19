use std::fs;
use std::fs::*;
use std::io::*;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;
use reqwest::blocking::Client;
use tar::Archive;
use xz::read::XzDecoder;

mod domain;
pub use crate::domain::CatalogItem;

mod catalog;
pub use crate::catalog::load_catalog;

mod directories;
pub use crate::directories::h2o_dir;

pub fn list() -> Result<()> {
    if let Some(h2o_dir) = h2o_dir() {
        let catalog = load_catalog();

        for key in catalog.keys() {
            let catalog_item = catalog.get(key).unwrap();
            println!("{}", catalog_item.name);
            let entrypoint = &h2o_dir.join("games").join(&catalog_item.entrypoint);
            if Path::new(entrypoint).exists() {
                println!("\tInstalled to: {}", entrypoint.display());
                println!("\tTo run: h2o run {key}");
            } else {
                println!("\th2o install {key}\n");
            }
        }
    }

    Ok(())
}

pub fn download(game: String) -> Result<()> {
    if let Some(h2o_dir) = h2o_dir() {
        let catalog = load_catalog();
        let catalog_item = catalog.get(&game).unwrap();
        let download_dir = h2o_dir.join("downloads");
        let download_file = download_dir.join(&catalog_item.download_file);
        let _ = fs::create_dir_all(&download_dir);

        println!("Downloading {game}. This may take a while...");

        let client = Client::builder()
            .timeout(Duration::from_secs(60 * 60))
            .build()
            .unwrap();
        let response = client.get(&catalog_item.download_source).send().unwrap();
        let buffer = response.bytes().expect("body invalid");
        let mut out = File::create(download_file).expect("Failed to create file");
        std::io::copy(&mut &buffer[..], &mut out).expect("failed to copy content");

        println!("Download completed.");
    }

    Ok(())
}

pub fn install(game: String) -> Result<()> {
    if let Some(h2o_dir) = h2o_dir() {
        let catalog = load_catalog();
        let catalog_item = catalog.get(&game).unwrap();
        let downloaded_file = h2o_dir.join("downloads").join(&catalog_item.download_file);

        if !Path::new(&downloaded_file).exists() {
            println!("'{game}' needs to be downloaded first:");
            println!("\n\th2o download fantasy\n");
        } else {
            let _ = unpack(h2o_dir.to_path_buf(), game);
        }
    }

    Ok(())
}

pub fn uninstall(game: String) -> Result<()> {
    if let Some(h2o_dir) = h2o_dir() {
        let catalog = load_catalog();
        let catalog_item = catalog.get(&game).unwrap();
        let game_dir = h2o_dir.join("games").join(&catalog_item.directory);
        let _ = remove_dir_all(game_dir);
    }

    Ok(())
}

pub fn unpack(h2o_dir: PathBuf, game: String) -> Result<()> {
    let source = h2o_dir
        .join("downloads")
        .join(format!("{game}-linux-x86_64.tar.xz"));
    let target = h2o_dir.join("games");

    println!("Installing {game}. This may take a while...");

    let tar_xz = File::open(source)?;
    let tar = XzDecoder::new(tar_xz);
    let _ = Archive::new(tar).unpack(target);

    println!("Finished installing {game}.");

    Ok(())
}

pub fn play(game: String) -> Result<()> {
    if let Some(h2o_dir) = h2o_dir() {
        let catalog = load_catalog();
        let game_target = &catalog
            .get(&game)
            .expect("Catalog item doesn't exist")
            .to_owned()
            .entrypoint;
        let target = h2o_dir.join("games").join(game_target);

        if Path::new(&target).exists() {
            println!("target: {}", target.display());
            println!("Running {game}...");
            let _ = Command::new(target).output();
        } else {
            println!("'{game}' does not exist. You can install it with:\n\n\th2o install {game}\n")
        }
    }

    Ok(())
}
