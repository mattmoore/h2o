use dirs::home_dir;
use std::collections::HashMap;
use std::fs::File;
use std::io::Result;
use std::path::{Path, PathBuf};
use std::process::Command;
use tar::Archive;
use xz::read::XzDecoder;

#[derive(Clone)]
pub struct CatalogItem {
    pub name: String,
    pub target: String,
    pub download_target: String,
}

pub fn catalog() -> Result<HashMap<String, CatalogItem>> {
    let mut catalog: HashMap<String, CatalogItem> = HashMap::new();
    catalog.insert(
        String::from("fantasy"),
        CatalogItem {
            name: String::from("Fantasy"),
            target: String::from("Fantasy/Fantasy.sh"),
            download_target: String::from("fantasy-linux-x86_64.tar.xz"),
        },
    );
    Ok(catalog)
}

pub fn list() -> Result<()> {
    if let Some(home) = home_dir() {
        let h2o_dir = &home.join(".h2o");
        let catalog = catalog()?;

        for item in catalog.keys() {
            let catalog_item = catalog.get(item).unwrap();
            println!("{}", catalog_item.name);
            let game_target = &h2o_dir.join("games").join(&catalog_item.target);
            if Path::new(game_target).exists() {
                println!("\tInstalled to: {}", game_target.display());
                println!("\tTo run: h2o run {}", item);
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
            .target;
        let target = h2o_dir.join("games").join(game_target);

        println!("target: {}", target.display());
        println!("Running {game}...");
        let _ = Command::new(target).output();
    }

    Ok(())
}
