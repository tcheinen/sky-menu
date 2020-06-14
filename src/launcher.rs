use crate::keyboard;

use qmetaobject::*;
use std::cell::RefCell;

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

        self.model.borrow_mut().push(ApplicationEntry {
            name: "HOWDY12345".to_string(),
        });

        self.model.borrow_mut().push(ApplicationEntry {
            name: "HOWDY2".to_string(),
        });
        self.model_changed();

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
        println!("{}", query);
    }
}

#[derive(Default, Clone, SimpleListItem)]
struct ApplicationEntry {
    pub name: String,
}
