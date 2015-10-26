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
        let w: Result<RecommendedWatcher, Error> = Watcher::new(tx);
        match w {
            Ok(watcher) => {
                Some(FilesystemNotifier {
                    rx: rx,
                    watcher: watcher,
                })
            },
            Err(_) => {
                None
            }
            
        }

    }

    fn process_event(event: Event) {
        let path = event.path.unwrap();
        let op = event.op.unwrap();

        println!("{:?} {}", op, path.to_str().unwrap());
    }

    pub fn process(&self) {
        loop {
            match self.rx.recv() {
                Ok(event) => FilesystemNotifier::process_event(event),
                Err(_) => {
                    return;
                }
            }
        }
    }
}

impl Notifier for FilesystemNotifier {
    fn add(&mut self, what: &str) {
        self.watcher.watch(what).ok();
    }
}
