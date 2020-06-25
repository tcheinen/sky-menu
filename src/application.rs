use freedesktop_entry_parser::parse_entry;
use std::path::{Path, PathBuf};

use std::collections::HashMap;

use crate::launcher::Application;
use crate::utility::get_xdg_data_dirs;
use cached::proc_macro::cached;
use std::{fs};

/// replace the format specifiers - most get replaced with nothing because they're for parameters or deprecated
/// %i is replaced with the Icon key, %c is replaced with the name, %k is replaced with the URI
#[cached]
fn filter_exec(exec: String, icon: String, name: String, uri: String) -> String {
    exec.replace("%F", "")
        .replace("%f", "")
        .replace("%U", "")
        .replace("%u", "")
        .replace("%D", "")
        .replace("%d", "")
        .replace("%v", "")
        .replace("%n", "")
        .replace("%N", "")
        .replace("%m", "")
        .replace("%i", &icon)
        .replace("%c", &name)
        .replace("%k", &uri)
}

#[cached]
pub fn parse_desktop_entry(filename: PathBuf) -> Application {
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
            let name = attributes
                .get("Name")
                .unwrap_or(&"".to_string())
                .to_string();

            let icon = attributes
                .get("Icon")
                .unwrap_or(&"application-x-executable".to_string())
                .to_string();

            let exec_pre = attributes
                .get("Exec")
                .unwrap_or(&"".to_string())
                .to_string();

            let exec = filter_exec(
                exec_pre,
                icon.clone(),
                name.clone(),
                filename.to_string_lossy().into(),
            );
            Application::new(name, icon, exec)
        })
        .next();
    results.unwrap_or(Application::default())
}

#[cached]
pub fn generate_application_list() -> HashMap<String, Application> {
    get_xdg_data_dirs()
        .iter()
        .map(|x| Path::new(x).join("applications"))
        .filter(|x| x.exists())
        .flat_map(|path| {
            fs::read_dir(path)
                .unwrap()
                .filter_map(|x| x.ok())
                .map(|x| parse_desktop_entry(x.path()))
                .map(|x| (x.name.clone(), x))
                .filter(|x| &x.1.name != "")
        })
        .collect::<HashMap<String, Application>>()
}

#[cfg(test)]
mod tests {
    use crate::application::{filter_exec, generate_application_list, parse_desktop_entry};
    use crate::launcher::Application;
    use std::path::PathBuf;

    #[test]
    fn it_filters_exec() {
        assert_eq!(
            filter_exec("howdy!".into(), "".into(), "".into(), "".into()),
            "howdy!"
        );
        assert_eq!(
            filter_exec("howdy!%F".into(), "".into(), "".into(), "".into()),
            "howdy!"
        );
        assert_eq!(
            filter_exec("ho%fwdy!".into(), "".into(), "".into(), "".into()),
            "howdy!"
        );
        assert_eq!(
            filter_exec(
                "%f%F%u%U%d%D%n%N%v%m".into(),
                "".into(),
                "".into(),
                "".into()
            ),
            ""
        );
        assert_eq!(
            filter_exec("%i".into(), "application".into(), "".into(), "".into()),
            "application"
        );
        assert_eq!(
            filter_exec("%c".into(), "".into(), "Files".into(), "".into()),
            "Files"
        );
        assert_eq!(
            filter_exec(
                "%k".into(),
                "".into(),
                "".into(),
                "/usr/share/applications/firefox.desktop".into()
            ),
            "/usr/share/applications/firefox.desktop"
        );
    }

    // #[test]
    /// relies on firefox being installed
    fn it_parses_apps() {
        assert_eq!(
            parse_desktop_entry(PathBuf::from("/usr/share/applications/firefox.desktop")),
            Application {
                name: "Firefox".into(),
                icon: "firefox".into(),
                exec: "/usr/lib/firefox/firefox ".into()
            }
        );
    }

    // #[test]
    /// relies on firefox and vim being installed
    fn it_generates_list() {
        // not going to test strict equality here because it'll vary so much. I'll just check a few
        let list = generate_application_list();
        assert!(list.values().any(|x| x.name == "Vim"));
        assert!(list.values().any(|x| x.name == "Firefox"));
    }
}
