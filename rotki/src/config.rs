use std::path::{Path, PathBuf};
use std::env;

use expanduser::expanduser;


fn get_xdg_home_dir() -> PathBuf {
    let path = match env::var("XDG_DATA_HOME") {
        Ok(path_str) => {
            Path::new(&path_str).to_path_buf()
        }
        Err(_) => {
            expanduser("~/.local/share").unwrap().to_path_buf()
        }
    };
    path
}

pub fn default_directory() -> PathBuf {
    let data_folder = "develop_data";
    let path = match env::consts::OS {
        "macos" => expanduser("~/Library/Application Support/rotki/")
                    .unwrap()
                    .join(data_folder)
                    .to_path_buf(),        
        _ => {
            let base = get_xdg_home_dir();
            base.join("rotki").join(data_folder)
        }
    };
    path
}