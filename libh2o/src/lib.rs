use dirs::home_dir;
use std::collections::HashMap;
use std::fs::{File, remove_dir_all};
use std::io::Result;
use std::path::{Path, PathBuf};
use std::process::Command;
use tar::Archive;
use xz::read::XzDecoder;

#[derive(Clone)]
pub struct CatalogItem {
    pub name: String,
    pub directory: String,
    pub download_source: String,
    pub entrypoint: String,
}

pub fn catalog() -> Result<HashMap<String, CatalogItem>> {
    let mut catalog: HashMap<String, CatalogItem> = HashMap::new();
    catalog.insert(
        String::from("fantasy"),
        CatalogItem {
            name: String::from("Fantasy"),
            directory: String::from("Fantasy"),
            download_source: String::from("fantasy-linux-x86_64.tar.xz"),
            entrypoint: String::from("Fantasy/Fantasy.sh"),
        },
    );
    Ok(catalog)
}

pub fn list() -> Result<()> {
    if let Some(home) = home_dir() {
        let h2o_dir = &home.join(".h2o");
        let catalog = catalog()?;

        for key in catalog.keys() {
            let catalog_item = catalog.get(key).unwrap();
            println!("{}", catalog_item.name);
            let entrypoint = &h2o_dir.join("games").join(&catalog_item.entrypoint);
            if Path::new(entrypoint).exists() {
                println!("\tInstalled to: {}", entrypoint.display());
                println!("\tTo run: h2o run {}", key);
            } else {
                println!("\th2o install {}\n", key);
            }
        }
    }

    Ok(())
}

pub fn install(game: String) -> Result<()> {
    if let Some(home) = home_dir() {
        let h2o_dir = &home.join(".h2o");
        let _ = unpack(h2o_dir.to_path_buf(), game);
    }

    Ok(())
}

pub fn uninstall(game: String) -> Result<()> {
    if let Some(home) = home_dir() {
        let h2o_dir = &home.join(".h2o");
        let catalog = catalog()?;
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

    println!("Installing {game}.");

    let tar_xz = File::open(source)?;
    let tar = XzDecoder::new(tar_xz);
    let _ = Archive::new(tar).unpack(target);

    println!("Finished installing {game}.");

    Ok(())
}

pub fn run(game: String) -> Result<()> {
    if let Some(home) = home_dir() {
        let h2o_dir = &home.join(".h2o");
        let game_target = catalog()?
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
