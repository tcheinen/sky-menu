mod application;
mod config;
mod error;
mod icon;
mod inotify_listener;
mod keyboard_listener;
mod launcher;
mod utility;

use crate::launcher::*;
use cstr::*;

use crate::config::Config;
use log::log;
use qmetaobject::*;

fn main() {
    env_logger::init();
    install_message_handler(logger);

    inotify_listener::listen();

    qml_register_type::<Launcher>(cstr!("Launcher"), 1, 0, cstr!("Launcher"));
    qml_register_type::<Config>(cstr!("Config"), 1, 0, cstr!("Config"));
    let mut engine = QmlEngine::new();
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
