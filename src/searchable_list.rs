use crate::application::generate_application_list;
use crate::config;
use crate::config::UsageCount;
use crate::icon::lookup_icon;
use crate::keyboard_listener;
use crate::keyboard_listener::KeyboardShortcut;
use crate::utility::get_running_applications;
use directories::ProjectDirs;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use itertools::Itertools;
use log::{error};
use qmetaobject::*;
use std::cell::RefCell;
use std::process::Command;

#[derive(PartialEq, Eq)]
enum ListType {
    Launcher,
    Switcher,
}

impl Default for ListType {
    fn default() -> Self {
        ListType::Launcher
    }
}

#[derive(QObject, Default)]
pub struct SearchableList {
    base: qt_base_class!(trait QObject),

    model: qt_property!(RefCell<SimpleListModel<Application>>; NOTIFY model_changed),
    visible: qt_property!(bool; NOTIFY visible_changed),
    selected: qt_property!(i32; NOTIFY selected_changed),
    focus: qt_property!(bool; NOTIFY focus_changed),
    model_len: qt_property!(i32; NOTIFY model_len_changed),
    hide_on_lost_focus: qt_property!(bool;),

    usage_count: UsageCount,

    list_type: ListType,

    setup: qt_method!(fn(&mut self)),
    up: qt_method!(fn(&mut self)),
    down: qt_method!(fn(&mut self)),
    launch: qt_method!(fn(&mut self)),
    hide: qt_method!(fn(&mut self)),
    show: qt_method!(fn(&mut self)),
    search: qt_method!(fn(&mut self, query: String)),
    icon: qt_method!(fn(&mut self, icon: String) -> QUrl),

    visible_changed: qt_signal!(),
    model_changed: qt_signal!(),
    selected_changed: qt_signal!(),
    focus_changed: qt_signal!(),
    model_len_changed: qt_signal!(),
}

impl SearchableList {
    fn setup(&mut self) {
        self.visible = false;
        self.visible_changed();

        self.focus = true;
        self.focus_changed();

        let launcher_qpointer = QPointer::from(&*self);
        let toggle_launcher = qmetaobject::queued_callback(move |()| {
            if let Some(qself) = launcher_qpointer.as_pinned() {
                qself.borrow_mut().list_type = ListType::Launcher;
                qself.borrow_mut().hide_on_lost_focus = true;
                qself.borrow_mut().visible = !qself.borrow().visible;
                qself.borrow().visible_changed();
                qself.borrow_mut().focus = true;
                qself.borrow().focus_changed();
                qself.borrow_mut().search("".into())
            }
        });

        let switcher_qpointer = QPointer::from(&*self);
        let show_switcher = qmetaobject::queued_callback(move |()| {
            if let Some(qself) = switcher_qpointer.as_pinned() {
                if qself.borrow().visible {
                    return;
                }
                qself.borrow_mut().hide_on_lost_focus = false;
                qself.borrow_mut().list_type = ListType::Switcher;
                qself.borrow_mut().visible = true;
                qself.borrow().visible_changed();
                qself.borrow_mut().focus = true;
                qself.borrow().focus_changed();
                qself.borrow_mut().search("".into())
            }
        });

        let hide_switcher_qpointer = QPointer::from(&*self);
        let hide_switcher = qmetaobject::queued_callback(move |()| {
            if let Some(qself) = hide_switcher_qpointer.as_pinned() {
                qself.borrow_mut().list_type = ListType::Switcher;
                qself.borrow_mut().visible = false;
                qself.borrow().visible_changed();
            }
        });
        if let Some(proj_dirs) =
            ProjectDirs::from(config::QUALIFIER, config::ORGANIZATION, config::APPLICATION)
        {
            self.usage_count = UsageCount::from(proj_dirs.data_dir().join("usage.json"))
        }

        self.search("".into());

        const SPACE_KEY_CODE: usize = 57;
        const TAB_KEY_CODE: usize = 15;
        const LCTRL_KEY_CODE: usize = 29;
        const LALT_KEY_CODE: usize = 56;

        let shortcuts = vec![
            KeyboardShortcut::new(
                |_key, x| x[LCTRL_KEY_CODE] && x[SPACE_KEY_CODE],
                toggle_launcher.clone(),
            ),
            KeyboardShortcut::new(|_key, x| x[LALT_KEY_CODE] && x[TAB_KEY_CODE], show_switcher),
            KeyboardShortcut::new(
                |key, x| key as usize == LALT_KEY_CODE && !x[LALT_KEY_CODE],
                hide_switcher,
            ),
        ];
        keyboard_listener::listen(shortcuts);
    }

    fn set_selected(&mut self, index: i32) {
        self.selected = index;
        let app = self.model.borrow()[self.selected as usize].clone();
        app.try_select();
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
        if self.model.borrow().row_count() == 0 {
            return;
        }

        let app = self.model.borrow()[self.selected as usize].clone();

        if app.try_exec() {
            self.usage_count.inc(&app.name)
        };
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
        self.set(
            self.get_app_list()
                .into_iter()
                .map(|x| (matcher.fuzzy_match(&x.name, &query).unwrap_or(0), x))
                .map(|(weight, app)| {
                    (
                        std::cmp::min(self.usage_count.get(&app.name) as i64 * 5, 50) + weight,
                        app,
                    )
                })
                .sorted_by(|a, b| b.0.cmp(&a.0))
                .map(|x| x.1)
                .collect(),
        );
        self.set_selected(0);
    }

    fn get_app_list(&self) -> Vec<Application> {
        match self.list_type {
            ListType::Launcher => generate_application_list()
                .into_iter()
                .map(|x| x.1)
                .collect(),
            ListType::Switcher => get_running_applications(),
        }
    }

    fn icon(&mut self, name: String) -> QUrl {
        let path = lookup_icon(name).unwrap_or(
            lookup_icon("application-x-executable".to_string()).unwrap_or("".to_string()),
        );
        QUrl::from(QString::from(path))
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

#[derive(Default, Debug, Clone, SimpleListItem, Eq, PartialEq, Ord, PartialOrd)]
pub struct Application {
    pub name: String,
    pub icon: String,
    pub exec: String,
    pub select: String,
}

impl Application {
    pub fn new(name: String, icon: String, exec: String, select: String) -> Self {
        Application {
            name,
            icon,
            exec,
            select,
        }
    }

    pub fn try_exec(&self) -> bool {
        if self.exec == "" {
            return false;
        }
        let cmd = self.exec.clone();
        Application::exec_string(cmd)
    }

    pub fn try_select(&self) -> bool {
        let cmd = self.select.clone();
        Application::exec_string(cmd)
    }

    fn exec_string(cmd: String) -> bool {
        if let Err(e) = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .arg("&")
            .arg("disown")
            .spawn()
        {
            error!("Couldn't launch program: {}", e);
            return false;
        }
        true
    }
}
