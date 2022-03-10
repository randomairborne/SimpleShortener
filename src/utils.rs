use dashmap::DashMap;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;

pub fn read_bincode(filename: &String) -> DashMap<String, String> {
    tracing::debug!("Reading bincode database file!");

    let target_path = Path::new(filename);
    if !target_path.exists() {
        tracing::warn!("Creating new database...");
        let new_map: DashMap<String, String> = DashMap::new();
        let encoded: Vec<u8> = bincode::serialize(&new_map).unwrap();
        std::fs::write(target_path, encoded)
            .expect("failed to write to database file: check permissions");
        return DashMap::new();
    }

    let f = OpenOptions::new()
        .read(true)
        .open(target_path)
        .expect("Failed to open file");

    let deserialized: DashMap<String, String> = match bincode::deserialize_from(f) {
        Ok(map) => map,
        Err(e) => {
            match *e {
                bincode::ErrorKind::Io(io_error) => panic!("I/O Error: {}", io_error),
                _ => panic!("Error deserializing database: {}", e),
            };
        }
    };
    deserialized
}

pub fn save_bincode<P: AsRef<Path>>(
    filename: P,
    map: &DashMap<String, String>,
) -> Result<(), bincode::Error> {
    let mut f = OpenOptions::new().write(true).read(false).open(filename)?;
    tracing::debug!("Saving bincode file...");
    tracing::trace!("with data {:#?}", map);
    let encoded: Vec<u8> = bincode::serialize(&map)?;
    f.write_all(&encoded)?;
    Ok(())
}
