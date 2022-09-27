// mod watcher;
use notify::{RecommendedWatcher, RecursiveMode, Watcher, Config};
use std::path::Path;

fn watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => println!("changed: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use tempfile::tempdir;
    use super::*;

    fn collect_watcher_events<F>(f: F) -> Vec<notify::Event>
        where F: FnOnce(&Path) {
        // create temp dir
        let dir = tempdir().expect("failed to create temp dir");

        // create channel
        let (tx, rx) = std::sync::mpsc::channel();

        {
            // create watcher that uses the channel
            let mut watcher = RecommendedWatcher::new(tx, Config::default()).expect("failed to create watcher");
            // watch that path
            watcher.watch(&dir.path(), RecursiveMode::Recursive).unwrap();
            // run the closure
            f(&dir.path());
        }
        // end of block, the watcher is dropped
        // rx will stop iteration and won't block

        rx.into_iter()
            .map(|x| x.unwrap())
            .collect()
    }

    #[test]
    fn do_nothing() {
        let events = collect_watcher_events(|_| ());
        assert_eq!(events, vec![]);
    }

    #[test]
    fn create_a_file() {
        let events = collect_watcher_events(|dir| {
            let path = dir.join("temp.txt");
            File::create(&path).expect("failed to create file");
        });
        // todo: what type of event should this return? normal `notify` events, or custom events?
        assert_eq!(events, vec![]);
    }
}
