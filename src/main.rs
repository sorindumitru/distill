use std::thread;
use std::sync::mpsc;

extern crate notify;

mod filesystem;

use self::filesystem::FilesystemNotifier;

pub struct Message;

pub trait Notifier {
    fn add(&mut self, what: &str);
}

fn main() {
    let (filesystem_tx, filesystem_rx) = mpsc::channel::<Message>();

    let filesystem_thread = thread::spawn( move || {
        let mut filesystem = FilesystemNotifier::new().unwrap();
        filesystem.add("/home/sorin/");
        filesystem.process();
    });
    
    let distill_thread = thread::spawn( move || {
        loop {
            let message = match filesystem_rx.recv() {
                Ok(m) => m,
                Err(e) => {
                    return;
                }
            };
        }
    });

    let dres = distill_thread.join();
    let fres = filesystem_thread.join();
}
