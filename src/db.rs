use crate::utils::save_bincode;
use crossbeam_channel::{unbounded, Sender};
use once_cell::sync::OnceCell;
use std::time::Duration;
use tracing::error;

static URL_FLUSH_TRIGGER: OnceCell<Sender<()>> = OnceCell::new();
const FIFTEEN_SECONDS: Duration = Duration::from_secs(15);

pub fn spawn_db_thread() {
    if URL_FLUSH_TRIGGER.get().is_some() {
        return;
    }

    std::thread::spawn(|| {
        let (tx, rx) = unbounded();

        let save_path = crate::CONFIG
            .get()
            .map(|x| x.database.clone())
            .expect("set config before spawning background save thread")
            .or_else(|| std::env::var("DATABASE_URI").ok())
            .expect("DATABASE_URI env var not set");

        let tx2 = tx.clone();

        let links = crate::URLS
            .get()
            .expect("set URLs before spawning background save thread");

        // this should be the very last thing before entering the loop, in case something else panics before
        // this will essentially cause the entire program to silently not save to disk if this is not the last thing
        // before the loop and something panics above
        URL_FLUSH_TRIGGER.set(tx).unwrap_or_else(|_| unreachable!());

        while rx.recv().is_ok() {
            if let Err(e) = save_bincode(&save_path, links) {
                error!("failed to flush to disk: {}", e);
                let tx3 = tx2.clone();
                std::thread::spawn(move || {
                    std::thread::sleep(FIFTEEN_SECONDS);
                    tx3.send(()).expect("background save thread panicked?");
                });
            }
        }
    });
}

/// Schedule the list of URLs to be flushed to disk at some time in the future.
pub fn flush_urls() {
    URL_FLUSH_TRIGGER
        .get()
        .expect("spawn db thread before scheduling a disk flush")
        .send(())
        .expect("thread panicked? should be impossible");
}
