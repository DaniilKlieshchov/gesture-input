use gesture_handler::GestureEventHandler;
use input::{Libinput, LibinputInterface};
use libc::{O_RDONLY, O_RDWR, O_WRONLY};
use nix::poll::{poll, PollFd, PollFlags};
use serde::Deserialize;
use std::fs::{File, OpenOptions};
use std::os::fd::AsRawFd;
use std::os::unix::{fs::OpenOptionsExt, io::OwnedFd};
use std::path::{Path, PathBuf};
mod gesture_handler;
mod configuration;
mod gesture_listener;
mod logger;

#[derive(Deserialize)]
struct Vars {
    seat: String,
    path: PathBuf,
}


struct Interface;

impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<OwnedFd, i32> {
        OpenOptions::new()
            .custom_flags(flags)
            .read((flags & O_RDONLY != 0) | (flags & O_RDWR != 0))
            .write((flags & O_WRONLY != 0) | (flags & O_RDWR != 0))
            .open(path)
            .map(|file| file.into())
            .map_err(|err| err.raw_os_error().unwrap())
    }
    fn close_restricted(&mut self, fd: OwnedFd) {
        drop(File::from(fd));
    }
}

fn main() {
    logger::setup_logger();
    let vars = match envy::from_env::<Vars>() {
        Ok(vars) => vars,
        Err(_error) => Vars { seat: "seat0".to_owned(), path: "config.yaml".into()},
    };
    let mut input = Libinput::new_with_udev(Interface);
    input.udev_assign_seat(vars.seat.as_str()).expect("Incorrect seat has been provided");
    let conf = configuration::get_configuration(&vars.path).expect("Can't read config");
    let mut gesture_handler = GestureEventHandler::new(conf.gestures);
    let pollfd = PollFd::new(input.as_raw_fd(), PollFlags::POLLIN);
    while poll(&mut [pollfd], -1).is_ok() {
        input.dispatch().unwrap();
        for event in &mut input {
            if let input::Event::Gesture(event) = event {
                gesture_handler.process_event(event)
            }
        }
    }
}
