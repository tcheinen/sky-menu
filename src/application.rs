use freedesktop_entry_parser::parse_entry;
use std::path::{Path, PathBuf};

use std::collections::HashMap;

use crate::launcher::Application;
use cached::proc_macro::cached;
use std::fs;

/// we just launch everything with the default arguments so we need to filter out any of the FreeDesktop arg specifiers
#[cached]
fn filter_exec(exec: String) -> String {
    exec.replace("%F", "")
        .replace("%f", "")
        .replace("%U", "")
        .replace("%u", "")
        .replace("%D", "")
        .replace("%d", "")
        .replace("%k", "")
        .replace("%v", "")
}

#[cached]
fn parse_desktop_entry(filename: PathBuf) -> Application {
    let results = parse_entry(&fs::read_to_string(filename.as_path()).unwrap().into_bytes())
        .filter_map(|y| y.ok())
        .filter(|y| y.title == b"Desktop Entry")
        .map(|y| {
            let attributes = y
                .attrs
                .iter()
                .map(|z| {
                    (
                        String::from_utf8_lossy(z.name).to_string(),
                        String::from_utf8_lossy(z.value).to_string(),
                    )
                })
                .filter(|x| x.0 == "Name" || x.0 == "Icon" || x.0 == "Exec")
                .collect::<HashMap<String, String>>();
            Application::new(
                attributes
                    .get("Name")
                    .unwrap_or(&"".to_string())
                    .to_string(),
                attributes
                    .get("Icon")
                    .unwrap_or(&"".to_string())
                    .to_string(),
                filter_exec(
                    attributes
                        .get("Exec")
                        .unwrap_or(&"".to_string())
                        .to_string(),
                ),
            )
        })
        .next();
    results.unwrap_or(Application::default())
}

#[cached]
pub fn generate_application_list() -> HashMap<String, Application> {
    fs::read_dir(Path::new("/usr/share/applications"))
        .unwrap()
        .filter_map(|x| x.ok())
        .map(|x| parse_desktop_entry(x.path()))
        .map(|x| (x.name.clone(), x))
        .filter(|x| &x.1.name != "")
        .collect::<HashMap<String, Application>>()
}
