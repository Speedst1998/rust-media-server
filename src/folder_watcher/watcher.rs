use log::info;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc, Mutex};

use crate::db::watched_folders_table::WatchedFolder;
/// Async, futures channel based event watching
pub struct FolderWatcher<'a> {
    folders_to_watch: &'a [&'a Path],
    watcher: Arc<Mutex<RecommendedWatcher>>,
    folder_content_update_event_receiver: Receiver<Result<Event, notify::Error>>,
    folder_list_event_receiver: Option<Receiver<WatchedFolder>>,
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
            folder_content_update_event_receiver: rx,
            folder_list_event_receiver: None,
            reset: false,
        })
    }

    pub async fn async_watch<P: AsRef<Path>>(&mut self, paths: Vec<P>) -> notify::Result<()> {
        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        
        tokio::join!(self.watch_folders(paths), self.watch_ui_events());

        Ok(())
    }

    pub fn create_sender(&mut self) -> mpsc::Sender<WatchedFolder> {
        let (sender, receiver) = mpsc::channel();
        self.folder_list_event_receiver = Some(receiver);
        sender
    }

    pub async fn watch_ui_events(&self) {
        self.test1s().await
    }

    async fn test1s(&self) {
        info!("called watched events");
        // TODO give a reason whny you have to set folder_list_event_receiver if None
        while let Some(watched_folder) = self.folder_list_event_receiver.as_ref().unwrap().iter().next() {
            info!("received event: folder path: {}", watched_folder.path);
            self.watcher
            .lock()
            .unwrap()
            .watch(watched_folder.path.as_ref(), RecursiveMode::Recursive)
            .unwrap();
        }
    }

    pub async fn watch_folders<P: AsRef<Path>>(&self, paths: Vec<P>) {
        self.test2s(paths).await
    }

    async fn test2s<P: AsRef<Path>>(&self, paths: Vec<P>) {
        let iterator = paths.iter();
        iterator.for_each(|path| {
            self.watcher
                .lock()
                .unwrap()
                .watch(path.as_ref(), RecursiveMode::Recursive)
                .unwrap();
        });

        while let Some(res) = self.folder_content_update_event_receiver.iter().next() {
            info!("in loop");
            match res {
                Ok(event) => info!("changed: {:?}", event),
                Err(e) => info!("watch error: {:?}", e),
            }
        }
    }
}
