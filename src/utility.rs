use std::env;

pub fn get_xdg_data_dirs() -> Vec<String> {
    env::var("XDG_DATA_DIRS")
        .unwrap_or("/usr/local/share/:/usr/share/".to_string())
        .split(":")
        .map(String::from)
        .collect()
}
