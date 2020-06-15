use crate::keyboard;

use crate::application::generate_application_list;
use qmetaobject::*;
use std::cell::RefCell;

use crate::icon::lookup_icon;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

use log::{error, warn};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::{env, fs};

use cached::proc_macro::cached;

#[derive(QObject, Default)]
pub struct Launcher {
    base: qt_base_class!(trait QObject),

    model: qt_property!(RefCell<SimpleListModel<Application>>; NOTIFY model_changed),
    visible: qt_property!(bool; NOTIFY visible_changed),
    height: qt_property!(i32; NOTIFY settings_changed),
    width: qt_property!(i32; NOTIFY settings_changed),
    selected: qt_property!(i32; NOTIFY selected_changed),
    focus: qt_property!(bool; NOTIFY focus_changed),
    model_len: qt_property!(i32; NOTIFY model_len_changed),

    usage_count: UsageCount,

    setup: qt_method!(fn(&mut self)),
    up: qt_method!(fn(&mut self)),
    down: qt_method!(fn(&mut self)),
    launch: qt_method!(fn(&mut self)),
    hide: qt_method!(fn(&mut self)),
    show: qt_method!(fn(&mut self)),
    search: qt_method!(fn(&mut self, query: String)),
    icon: qt_method!(fn(&mut self, icon: String) -> QUrl),

    visible_changed: qt_signal!(),
    settings_changed: qt_signal!(),
    model_changed: qt_signal!(),
    selected_changed: qt_signal!(),
    focus_changed: qt_signal!(),
    model_len_changed: qt_signal!(),
}

impl Launcher {
    fn setup(&mut self) {
        self.visible = true;
        self.visible_changed();

        self.focus = true;
        self.focus_changed();

        self.height = 500;
        self.width = 400;

        self.settings_changed();

        self.set(
            generate_application_list()
                .keys()
                .map(|x| {
                    generate_application_list()
                        .get(&x.to_string())
                        .map_or(Application::default(), |x| x.clone())
                })
                .collect(),
        );

        let self_qpointer = QPointer::from(&*self);
        let toggle_visibility = qmetaobject::queued_callback(move |()| {
            if let Some(qself) = self_qpointer.as_pinned() {
                qself.borrow_mut().visible = !qself.borrow().visible;
                qself.borrow().visible_changed();
                qself.borrow_mut().focus = true;
                qself.borrow().focus_changed();
            }
        });

        let mut data_dir = get_data_dir();
        data_dir.push("usage.json");

        self.usage_count = UsageCount::from(data_dir);

        // keycode 29 -> lctrl, 97 -> rctrl,  57 -> space
        let predicate = |state: [bool; 256]| (state[29] || state[97]) && state[57];
        keyboard::listen(predicate, toggle_visibility);
    }

    fn set_selected(&mut self, index: i32) {
        self.selected = index;
        self.selected_changed();
    }

    fn up(&mut self) {
        if self.model.borrow().row_count() == 0 {
            return;
        }
        let max_index = self.model.borrow().row_count();
        self.set_selected((self.selected - 1).rem_euclid(max_index))
    }

    fn down(&mut self) {
        if self.model.borrow().row_count() == 0 {
            return;
        }
        let max_index = self.model.borrow().row_count();
        self.set_selected((self.selected + 1).rem_euclid(max_index));
    }

    fn launch(&mut self) {
        use std::process::Command;
        if self.model.borrow().row_count() == 0 {
            return;
        }

        let app = self.model.borrow()[self.selected as usize].clone();

        if let Err(e) = Command::new("sh")
            .arg("-c")
            .arg(app.exec.clone())
            .arg("&")
            .arg("disown")
            .spawn()
        {
            error!("Couldn't launch program: {}", e);
        }
        self.usage_count.inc(&app.name);
        self.hide();
    }

    fn hide(&mut self) {
        self.visible = false;
        self.visible_changed();
    }

    fn show(&mut self) {
        self.visible = true;
        self.visible_changed();
        self.set_selected(0);
    }
    fn search(&mut self, query: String) {
        let matcher = SkimMatcherV2::default();
        let mut list: Vec<(i64, String)> = generate_application_list()
            .keys()
            .map(|x| (matcher.fuzzy_match(x, &query), x))
            .filter_map(|x| {
                if x.0.is_some() {
                    Some((x.0.unwrap(), x.1.to_string()))
                } else {
                    None
                }
            })
            .collect();
        list.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        self.set(
            list.iter()
                .map(|x| {
                    generate_application_list()
                        .get(&x.1.to_string())
                        .map_or(Application::default(), |x| x.clone())
                })
                .map(|x| x.clone())
                .collect(),
        );
        self.set_selected(0);
    }

    fn icon(&mut self, name: String) -> QUrl {
        QUrl::from(QString::from(lookup_icon(name).unwrap_or("".to_string())))
    }

    fn set(&mut self, list: Vec<Application>) {
        self.model
            .borrow_mut()
            .reset_data(list.into_iter().take(9).collect());
        self.model_len = self.model.borrow().row_count();
        self.model_changed();
        self.model_len_changed();
    }
}

#[cached]
fn get_data_dir() -> PathBuf {
    match env::var("XDG_DATA_HOME") {
        Ok(x) => PathBuf::from(x),
        Err(_) => match env::var("HOME") {
            Ok(x) => Path::new(&x).join(Path::new(".local/share")),
            Err(_) => {
                warn!("Couldn't resolve $XDG_DATA_HOME or $HOME, using cwd for usage database");
                PathBuf::from(".")
            }
        },
    }
    .join(Path::new("launcher"))
    .to_path_buf()
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UsageCount {
    store: HashMap<String, i32>,
}

impl From<PathBuf> for UsageCount {
    fn from(x: PathBuf) -> Self {
        let json = fs::read_to_string(&x).unwrap_or("{}".to_string());
        UsageCount {
            store: serde_json::from_str(&json).unwrap_or(HashMap::new()),
        }
    }
}

impl UsageCount {
    pub fn inc(&mut self, app: &str) {
        self.set(app, self.get(app) + 1)
    }
    pub fn get(&self, app: &str) -> i32 {
        self.store.get(app).unwrap_or(&0).clone()
    }

    pub fn set(&mut self, app: &str, val: i32) {
        self.store.insert(app.to_string(), val);
        let json = match serde_json::to_string(&self.store) {
            Ok(x) => x,
            Err(e) => {
                warn!("Couldn't serialize usage database to json: {}", e);
                return;
            }
        };
        let mut out_loc = get_data_dir();

        // TODO handle errors here aka ignore them and warn in console
        fs::create_dir_all(out_loc.as_path());
        out_loc.push(Path::new("usage.json"));
        fs::write(out_loc.as_path(), json);
    }
}

#[derive(Default, Debug, Clone, SimpleListItem)]
pub struct Application {
    pub name: String,
    pub icon: String,
    pub exec: String,
}

impl Application {
    pub fn new(name: String, icon: String, exec: String) -> Self {
        Application { name, icon, exec }
    }
}
