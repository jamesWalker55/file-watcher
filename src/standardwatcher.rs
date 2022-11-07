use notify::{RecommendedWatcher, EventHandler, Config, Watcher};

use std::sync::mpsc::Sender;
use notify::WatcherKind::ReadDirectoryChangesWatcher;

enum Error {
    SystemNotSupported
}

enum TargetKind {
    File,
    Folder,
    /// An item which specific kind is known but cannot be represented otherwise.
    Other,
    /// An item which specific kind is not known.
    Unknown,
}

enum Event {
    Create(&path, TargetKind),
    Remove(&path, TargetKind),
    Modify(&path, TargetKind),
    Rename(&path, &path, TargetKind),
}

struct StandardWatcher {
    base_watcher: dyn notify::Watcher,
    channel: Sender<EventLoopMsg>,
}

impl StandardWatcher {
    fn new<F: EventHandler>(event_handler: F) -> Result<Self, Error> {
        if RecommendedWatcher::kind() != ReadDirectoryChangesWatcher {
            return Err(Error::SystemNotSupported);
        }

        Ok(StandardWatcher {
            base_watcher: RecommendedWatcher(event_handler, Config::default()),
            channel: event_handler,
        })
    }
}