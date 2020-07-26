use qmetaobject::*;

pub static QUALIFIER: &str = "com.teddyheinen";
pub static ORGANIZATION: &str = "Teddy Heinen";
pub static APPLICATION: &str = "sky-menu";

#[derive(QObject, Default)]
pub struct Config {
    base: qt_base_class!(trait QObject),

    height: qt_property!(i32; NOTIFY config_changed),
    width: qt_property!(i32; NOTIFY config_changed),

    setup: qt_method!(fn(&mut self)),

    config_changed: qt_signal!(),
}

impl Config {
    // TODO make this pull from a file
    fn setup(&mut self) {
        self.height = 500;
        self.width = 400;

        self.config_changed();
    }
}
