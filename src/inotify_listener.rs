use crate::application::generate_application_list;
use crate::utility::get_xdg_application_dirs;
use cached::Cached;
use inotify::{EventMask, Inotify, WatchMask};
use std::path::Path;
use std::{fs, thread};
/// start inotify listeners - primarily used to invalidate parts of the cache
pub fn listen() {
    let mut inotify = Inotify::init().expect("Error while initializing inotify instance");
    get_xdg_application_dirs().for_each(|x| {
        inotify
            .add_watch(x.clone(), WatchMask::CREATE)
            .expect("Error while watching");
        fs::read_dir(x)
            .expect("error while reading xdg app dir files")
            .filter_map(|y| y.ok())
            .for_each(|y| {
                inotify
                    .add_watch(y.path(), WatchMask::MODIFY)
                    .expect("Error while watching");
            })
    });
    thread::spawn(move || loop {
        generate_application_list();
        let mut buffer = [0; 1024];
        let mut events = inotify
            .read_events_blocking(&mut buffer)
            .expect("Error while reading events");
        if events.any(|x| x.mask == EventMask::CREATE) {
            crate::application::GENERATE_APPLICATION_LIST
                .lock()
                .unwrap()
                .cache_reset();
        }
        if events.any(|x| x.mask == EventMask::MODIFY) {
            events.filter(|x| x.name.is_some()).for_each(|x| {
                let mut cache = crate::application::PARSE_DESKTOP_ENTRY.lock().unwrap();
                get_xdg_application_dirs()
                    .map(|y| y.join(Path::new(x.name.unwrap())))
                    .for_each(|y| {
                        // TODO confirm that this works by making sure its comparing equality and not identity
                        cache.cache_remove(&y);
                    });
            })
        }
    });
}
