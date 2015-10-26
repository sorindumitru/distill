use std::sync::mpsc;

use notify::*;

use ::Notifier;

pub struct FilesystemNotifier {
    rx: mpsc::Receiver<Event>,
    watcher: RecommendedWatcher,
}

impl FilesystemNotifier {
    pub fn new() -> Option<FilesystemNotifier> {
        let (tx, rx) = mpsc::channel::<Event>();
        let mut w: Result<RecommendedWatcher, Error> = Watcher::new(tx);
        match w {
            Ok(mut watcher) => {
                Some(FilesystemNotifier {
                    rx: rx,
                    watcher: watcher,
                })
            },
            Err(e) => {
                None
            }
            
        }

    }

    fn process_event(event: Event) {
        let path = event.path.unwrap();
        match event.op.unwrap() {
            op::CREATE => println!("CREATE {}", path.to_str().unwrap()),
            op::REMOVE => println!("REMOVE {}", path.to_str().unwrap()),
            op::RENAME => println!("RENAME {}", path.to_str().unwrap()),
            op::CHMOD => println!("CHMOD {}", path.to_str().unwrap()),
            op::WRITE => println!("WRITE {}", path.to_str().unwrap()),
            _ => println!("Something Else"),
        }
    }

    pub fn process(&self) {
        loop {
            match self.rx.recv() {
                Ok(event) => FilesystemNotifier::process_event(event),
                Err(e) => {
                    return;
                }
            }
        }
    }
}

impl Notifier for FilesystemNotifier {
    fn add(&mut self, what: &str) {
        self.watcher.watch(what);
    }
}
