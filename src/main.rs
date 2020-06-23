mod application;
mod error;
mod icon;
mod keyboard;
mod launcher;
mod utility;

use crate::launcher::*;
use cstr::*;

use crate::application::generate_application_list;
use crate::utility::get_xdg_data_dirs;
use cached::once_cell::sync::OnceCell;
use cached::Cached;
use inotify::{Inotify, WatchMask};
use log::log;
use qmetaobject::*;
use std::path::Path;
use std::{thread, time};
fn main() {
    env_logger::init();
    install_message_handler(logger);

    qml_register_type::<Launcher>(cstr!("Launcher"), 1, 0, cstr!("Launcher"));
    let mut engine = QmlEngine::new();
    {
        let mut inotify = Inotify::init().expect("Error while initializing inotify instance");
        get_xdg_data_dirs()
            .iter()
            .map(|x| Path::new(x).join("applications"))
            .filter(|x| x.exists())
            .for_each(|x| {
                inotify
                    .add_watch(x, WatchMask::CREATE)
                    .expect("Error while watching");
            });
        thread::spawn(move || loop {
            generate_application_list();
            let mut buffer = [0; 1024];
            let events = inotify
                .read_events_blocking(&mut buffer)
                .expect("Error while reading events");
            crate::application::GENERATE_APPLICATION_LIST
                .lock()
                .unwrap()
                .cache_reset();
        });
    }
    engine.load_data(include_str!("main.qml").into());
    engine.exec();
}

extern "C" fn logger(qt_msg_type: QtMsgType, context: &QMessageLogContext, msg: &QString) {
    let level = match qt_msg_type {
        QtMsgType::QtCriticalMsg | QtMsgType::QtFatalMsg => log::Level::Error,
        QtMsgType::QtWarningMsg => log::Level::Warn,
        QtMsgType::QtInfoMsg => log::Level::Info,
        QtMsgType::QtDebugMsg => log::Level::Debug,
    };
    log!(
        level,
        "{:?}, file: {}:{} - category: {} - function: {} - [{}]",
        qt_msg_type,
        context.file(),
        context.line(),
        context.category(),
        context.function(),
        msg
    );
}
