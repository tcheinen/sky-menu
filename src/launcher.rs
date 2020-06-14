use crate::keyboard;

use crate::search::APPLICATIONS;
use qmetaobject::*;
use std::cell::RefCell;





use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

#[derive(QObject, Default)]
pub struct Launcher {
    base: qt_base_class!(trait QObject),

    model: qt_property!(RefCell<SimpleListModel<ApplicationEntry>>; NOTIFY model_changed),
    visible: qt_property!(bool; NOTIFY visible_changed),
    height: qt_property!(i32; NOTIFY settings_changed),
    width: qt_property!(i32; NOTIFY settings_changed),
    selected: qt_property!(i32; NOTIFY selected_changed),
    focus: qt_property!(bool; NOTIFY focus_changed),

    setup: qt_method!(fn(&mut self)),
    up: qt_method!(fn(&mut self)),
    down: qt_method!(fn(&mut self)),
    launch: qt_method!(fn(&mut self)),
    hide: qt_method!(fn(&mut self)),
    search: qt_method!(fn(&mut self, query: String)),

    visible_changed: qt_signal!(),
    settings_changed: qt_signal!(),
    model_changed: qt_signal!(),
    selected_changed: qt_signal!(),
    focus_changed: qt_signal!(),
}

impl Launcher {
    fn setup(&mut self) {
        self.visible = true;
        self.visible_changed();

        self.focus = true;
        self.focus_changed();

        self.height = 500;
        self.width = 300;

        self.settings_changed();

        self.set(
            APPLICATIONS
                .keys()
                .map(|x| ApplicationEntry::new(x.to_string()))
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

        // keycode 29 -> lctrl, 97 -> rctrl,  57 -> space
        let predicate = |state: [bool; 256]| (state[29] || state[97]) && state[57];
        keyboard::listen(predicate, toggle_visibility);
    }

    fn up(&mut self) {
        if self.model.borrow().row_count() == 0 {
            return;
        }
        self.selected = (self.selected - 1).rem_euclid(self.model.borrow().row_count());
        self.selected_changed();
    }

    fn down(&mut self) {
        if self.model.borrow().row_count() == 0 {
            return;
        }
        self.selected = (self.selected + 1).rem_euclid(self.model.borrow().row_count());
        self.selected_changed();
    }

    fn launch(&mut self) {
        if self.model.borrow().row_count() == 0 {
            return;
        }
        println!("{}", self.model.borrow()[self.selected as usize].name);
    }

    fn hide(&mut self) {
        if self.model.borrow().row_count() == 0 {
            return;
        }
        self.visible = false;
        self.visible_changed();
    }

    fn search(&mut self, query: String) {
        let matcher = SkimMatcherV2::default();
        let mut list: Vec<(i64, String)> = APPLICATIONS
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
                .map(|x| ApplicationEntry::new(x.1.to_string()))
                .collect(),
        );
        println!("{}", query);
    }

    fn set(&mut self, list: Vec<ApplicationEntry>) {
        self.model.borrow_mut().reset_data(list);
        self.model_changed();
    }
}

#[derive(Default, Clone, SimpleListItem)]
struct ApplicationEntry {
    pub name: String,
}

impl ApplicationEntry {
    pub fn new(name: String) -> Self {
        ApplicationEntry { name }
    }
}
