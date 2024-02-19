use dirs::home_dir;

use std::path::PathBuf;

pub fn h2o_dir() -> Option<PathBuf> {
    match home_dir() {
        Some(home) => Some(home.join(".h2o")),
        None => panic!("Problem finding .h2o directory."),
    }
}
