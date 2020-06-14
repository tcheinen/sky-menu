use input::event::keyboard::{KeyState, KeyboardEventTrait};
use input::{Libinput, LibinputInterface};
use std::fs::{File, OpenOptions};
use std::os::unix::io::{FromRawFd, IntoRawFd, RawFd};
use std::path::Path;

use std::{thread, time};

extern crate libc;

use input::event::Event::Keyboard;

use libc::{O_RDONLY, O_RDWR, O_WRONLY};
use std::os::unix::fs::OpenOptionsExt;

/// listen for keyboard events
/// predicate should determine based on the passed state if executor is called
/// and executor should do whatever you want on that event
pub fn listen(
    predicate: impl Fn([bool; 256]) -> bool + Send + 'static,
    executor: impl Fn(()) + Send + 'static,
) {
    thread::spawn(move || {
        let mut state = [false; 256];
        let mut input = Libinput::new_with_udev(Interface);
        input.udev_assign_seat("seat0").unwrap();
        loop {
            input.dispatch().unwrap();
            for event in &mut input {
                if let Keyboard(key_event) = event {
                    let key = key_event.key();
                    state[key as usize] = match key_event.key_state() {
                        KeyState::Pressed => true,
                        KeyState::Released => false,
                    };
                    if predicate(state) {
                        executor(());
                    }
                }
            }

            // Libinput::dispatch is nonblocking so we need to wait a bit before checking again for the sake of our poor cpu
            thread::sleep(time::Duration::from_millis(100));
        }
    });
}

struct Interface;

impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<RawFd, i32> {
        OpenOptions::new()
            .custom_flags(flags)
            .read((flags & O_RDONLY != 0) | (flags & O_RDWR != 0))
            .write((flags & O_WRONLY != 0) | (flags & O_RDWR != 0))
            .open(path)
            .map(|file| file.into_raw_fd())
            .map_err(|err| err.raw_os_error().unwrap())
    }
    fn close_restricted(&mut self, fd: RawFd) {
        unsafe {
            File::from_raw_fd(fd);
        }
    }
}
