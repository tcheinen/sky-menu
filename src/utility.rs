use std::env;
use std::path::{Path, PathBuf};

// FIXME this should return an iterator but im running into borrow checker troubles
pub fn get_xdg_data_dirs() -> Vec<String> {
    env::var("XDG_DATA_DIRS")
        .unwrap_or("/usr/local/share/:/usr/share/".into())
        .split(":")
        .map(String::from)
        .collect()
}

pub fn get_xdg_application_dirs() -> impl Iterator<Item = PathBuf> {
    get_xdg_data_dirs()
        .into_iter()
        .map(|x| PathBuf::from(x).join("applications"))
        .filter(|x| x.exists())
}
