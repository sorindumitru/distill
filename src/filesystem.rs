use inotify::INotify;
use inotify::wrapper::Event;
use inotify::ffi::*;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use ::Notifier;

pub struct FilesystemNotifier {
    ino: INotify,
    watches: HashMap<i32, String>,
}

impl FilesystemNotifier {
    pub fn new() -> Option<FilesystemNotifier> {
        let ino_res = INotify::init();

        match ino_res {
            Ok(ino) => {
                Some(FilesystemNotifier {
                    ino: ino,
                    watches: HashMap::new(),
                })
            },
            Err(_) => {
                None
            }
        }
    }

    fn process_event(event: &Event) {
        if event.is_access() {
            println!("Accesing {}:{}", event.name, event.cookie);
        }

        if event.is_modify() {
            println!("Modified {}:{}", event.name, event.cookie);
        }

        if event.is_attrib() {
            println!("Attrib {}:{}", event.name, event.cookie);
        }

        if event.is_close_write() {
            println!("Close write {}:{}", event.name, event.cookie);
        }

        if event.is_close_nowrite() {
            println!("Close nowrite {}:{}", event.name, event.cookie);
        }

        if event.is_open() {
            println!("Open {}:{}", event.name, event.cookie);
        }

        if event.is_moved_from() {
            println!("Is moved from {}:{}", event.name, event.cookie);
        }

        if event.is_moved_to() {
            println!("Is moved to {}:{}", event.name, event.cookie);
        }

        if event.is_create() {
            println!("Is create {}:{}", event.name, event.cookie);
        }

        if event.is_delete() {
            println!("Is delete {}:{}", event.name, event.cookie);
        }
    }

    pub fn process(&mut self) {
        loop {
            let events = self.ino.wait_for_events().unwrap();

            for event in events.iter() {
                FilesystemNotifier::process_event(event);
            }
        }
    }
}

impl Notifier for FilesystemNotifier {
    fn add(&mut self, dir: &str) {
        // TODO: Check the result and something with it
        let watch = match self.ino.add_watch(Path::new(dir), IN_ALL_EVENTS) {
            Ok(watch) => watch,
            Err(_) => return,
        };

        self.watches.insert(watch, String::from(dir));
    }

    fn add_recursive(&mut self, dir: &str) {
        let meta = match fs::metadata(dir) {
            Ok(meta) => meta,
            Err(_) => return,
        };

        if !meta.is_dir() {
            return;
        }

        self.add(dir);

        let dirs = match fs::read_dir(dir) {
            Ok(dirs) => dirs,
            Err(_) => return,
        };

        for entry in dirs {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => return,
            };
            match entry.path().to_str() {
                None => return,
                Some(entry) => self.add_recursive(entry),
            };
        }
    }
}
