use freedesktop_entry_parser::parse_entry;
use std::path::Path;

use std::collections::HashMap;

use crate::launcher::Application;
use std::fs;

lazy_static! {
    pub static ref APPLICATIONS: HashMap<String, Application> = generate_application_list();
}

/// we just launch everthing with the default arguments so we need to filter out any of the FreeDesktop arg specifiers
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

fn parse_desktop_entry(filename: &Path) -> Application {
    let results = parse_entry(&fs::read_to_string(filename).unwrap().into_bytes())
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

pub fn generate_application_list() -> HashMap<String, Application> {
    fs::read_dir(Path::new("/usr/share/applications"))
        .unwrap()
        .filter_map(|x| x.ok())
        .map(|x| parse_desktop_entry(x.path().as_path()))
        .map(|x| (x.name.clone(), x))
        .collect::<HashMap<String, Application>>()
}
