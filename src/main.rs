mod application;
mod error;
mod icon;
mod keyboard;
mod launcher;

use crate::launcher::*;
use cstr::*;

use log::log;
use qmetaobject::*;

use crate::application::generate_application_list;
fn main() {
    env_logger::init();
    install_message_handler(logger);

    qml_register_type::<Launcher>(cstr!("Launcher"), 1, 0, cstr!("Launcher"));
    let mut engine = QmlEngine::new();

    generate_application_list();

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
