use freedesktop_entry_parser::parse_entry;
use std::path::{Path};

use std::collections::HashMap;

use std::fs;

lazy_static! {
    pub static ref APPLICATIONS: HashMap<String, HashMap<String, String>> =
        generate_application_list();
}

pub fn generate_application_list() -> HashMap<String, HashMap<String, String>> {
    fs::read_dir(Path::new("/usr/share/applications"))
        .unwrap()
        .filter_map(|x| x.ok())
        .filter_map(|x| fs::read_to_string(x.path()).ok())
        .map(|x| x.into_bytes())
        .flat_map(|x| {
            parse_entry(&x)
                .filter_map(|y| y.ok())
                .filter(|y| y.title == b"Desktop Entry")
                .map(|y| {
                    y.attrs
                        .iter()
                        .map(|z| {
                            (
                                String::from_utf8_lossy(z.name).to_string(),
                                String::from_utf8_lossy(z.value).to_string(),
                            )
                        })
                        .collect::<HashMap<String, String>>()
                })
                .filter_map(|y: HashMap<String, String>| match y.get("Name") {
                    Some(name) => Some((name.to_string(), y)),
                    _ => None,
                })
                .collect::<Vec<(String, HashMap<String, String>)>>()
        })
        .collect::<HashMap<String, HashMap<String, String>>>()
}
