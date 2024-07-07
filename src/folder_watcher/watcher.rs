use log::info;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc, Mutex};
/// Async, futures channel based event watching
pub struct FolderWatcher<'a> {
    folders_to_watch: &'a [&'a Path],
    watcher: Arc<Mutex<RecommendedWatcher>>,
    event_receiver: Receiver<Result<Event, notify::Error>>,
    reset: bool,
}

impl<'a> FolderWatcher<'a> {
    pub fn new() -> Result<FolderWatcher<'a>, notify::Error> {
        let (tx, rx) = mpsc::channel();

        // Automatically select the best implementation for your platform.
        // You can also access each implementation directly e.g. INotifyWatcher.
        let watcher = Arc::new(Mutex::new(RecommendedWatcher::new(
            move |res| tx.send(res).unwrap(),
            Config::default(),
        )?));

        Ok(FolderWatcher {
            folders_to_watch: &[],
            watcher,
            event_receiver: rx,
            reset: false,
        })
    }

    pub async fn async_watch<P: AsRef<Path>>(&mut self, paths: Vec<P>) -> notify::Result<()> {
        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        let iterator = paths.iter();
        iterator.for_each(|path| {
            self.watcher
                .lock()
                .unwrap()
                .watch(path.as_ref(), RecursiveMode::Recursive)
                .unwrap();
        });

        while let Some(res) = self.event_receiver.iter().next() {
            match res {
                Ok(event) => info!("changed: {:?}", event),
                Err(e) => info!("watch error: {:?}", e),
            }
        }

        Ok(())
    }
}
