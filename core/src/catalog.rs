use std::collections::HashMap;

pub use crate::domain::CatalogItem;

pub fn load_catalog() -> HashMap<String, CatalogItem> {
    let mut catalog: HashMap<String, CatalogItem> = HashMap::new();
    catalog.insert(
        String::from("fantasy"),
        CatalogItem {
            name: String::from("Fantasy"),
            directory: String::from("Fantasy"),
            download_source: String::from(
                "http://143.244.222.68/fantasy-linux-x86_64.tar.xz",
            ),
            download_file: String::from("fantasy-linux-x86_64.tar.xz"),
            entrypoint: String::from("Fantasy/Fantasy.sh"),
        },
    );
    catalog
}
