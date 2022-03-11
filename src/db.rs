use crate::utils::save_bincode;
use once_cell::sync::OnceCell;
use parking_lot::Mutex;

static URL_DB_FILE_LOCK: OnceCell<Mutex<String>> = OnceCell::new();

pub fn init_db_storage() {
    if URL_DB_FILE_LOCK.get().is_some() {
        return;
    }

    let save_path = crate::CONFIG
        .get()
        .map(|x| x.database.clone())
        .expect("set config before spawning background save thread")
        .or_else(|| std::env::var("DATABASE_URI").ok())
        .expect("DATABASE_URI env var not set");
    URL_DB_FILE_LOCK
        .set(Mutex::new(save_path))
        .unwrap_or_else(|_| unreachable!());
}

/// Flush the list of URLs to disk.
pub fn flush_urls() -> Result<(), bincode::Error> {
    let path = URL_DB_FILE_LOCK
        .get()
        .expect("call `init_db_storage()` before trying to flush URLs")
        .lock();
    let links = crate::URLS
        .get()
        .expect("set URLs before spawning background save thread");
    save_bincode(path.clone(), links)?;
    Ok(())
}
