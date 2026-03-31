use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::Duration;

use notify_debouncer_mini::notify::{self, RecursiveMode};
use notify_debouncer_mini::{Config, DebouncedEventKind, new_debouncer_opt};

pub fn watch_non_recursive(
    path: impl AsRef<Path>,
    timeout: Duration,
    poll_interval: Option<Duration>,
) -> notify::Result<mpsc::Receiver<Result<PathBuf, notify::Error>>> {
    let (tx, rx) = mpsc::channel();

    let mut backend_config = notify::Config::default();
    if let Some(poll_interval) = poll_interval {
        backend_config = backend_config.with_poll_interval(poll_interval);
    }
    let debouncer_config =
        Config::default().with_timeout(timeout).with_notify_config(backend_config);

    let mut debouncer = new_debouncer_opt::<_, notify::PollWatcher>(debouncer_config, tx).unwrap();
    debouncer.watcher().watch(path.as_ref(), RecursiveMode::NonRecursive).unwrap();

    let (tx_mapped, rx_mapped) = mpsc::channel();

    // Cursed?
    std::thread::spawn(move || {
        for msg in rx {
            match msg {
                Ok(events) => {
                    for event in events {
                        if event.kind == DebouncedEventKind::Any {
                            if tx_mapped.send(Ok(event.path)).is_err() {
                                return;
                            }
                        }
                    }
                }
                Err(err) => {
                    if tx_mapped.send(Err(err)).is_err() {
                        return;
                    }
                }
            }
        }
    });

    Ok(rx_mapped)
}
