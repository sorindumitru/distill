use std::thread;
use std::sync::mpsc;

extern crate inotify;

mod filesystem;

use self::filesystem::FilesystemNotifier;

pub struct Message;

pub trait Notifier {
    fn add(&mut self, what: &str);
}

fn process_message(_: Message) {
}

fn main() {
    let (_, filesystem_rx) = mpsc::channel::<Message>();

    let filesystem_thread = thread::spawn( move || {
        let mut filesystem = FilesystemNotifier::new().unwrap();
        filesystem.add("/home/sorin/");
        filesystem.process();
    });

    let distill_thread = thread::spawn( move || {
        loop {
            let message = match filesystem_rx.recv() {
                Ok(m) => m,
                Err(_) => {
                    return;
                }
            };

            process_message(message);
        }
    });

    let dres = distill_thread.join();
    dres.ok();
    let fres = filesystem_thread.join();
    fres.ok();
}
