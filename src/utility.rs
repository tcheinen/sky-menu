use std::env;
use std::path::PathBuf;

pub fn get_xdg_data_dirs() -> impl Iterator<Item = PathBuf> {
    env::var("XDG_DATA_DIRS")
        .unwrap_or("/usr/local/share/:/usr/share/".into())
        .split(":")
        .map(PathBuf::from)
        .collect::<Vec<PathBuf>>() // FIXME why do i need to collect here
        .into_iter()
}

pub fn get_xdg_application_dirs() -> impl Iterator<Item = PathBuf> {
    get_xdg_data_dirs()
        .into_iter()
        .map(|x| PathBuf::from(x).join("applications"))
        .filter(|x| x.exists())
}
