use dashmap::DashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

pub fn read_bincode(filename: &String) -> DashMap<String, String> {
    tracing::debug!("Reading bincode database file!");
    let mut f = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(filename)
        .expect("Failed to open file");

    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read_exact(&mut buffer).expect("buffer overflow");
    let deserialized: DashMap<String, String> = match bincode::deserialize(&buffer[..]) {
        Ok(map) => map,
        Err(e) => {
            match *e {
                bincode::ErrorKind::Io(io_error) => {
                    if io_error.kind() == io::ErrorKind::UnexpectedEof {
                        tracing::warn!("Creating new database...");
                        let new_map: DashMap<String, String> = DashMap::new();
                        let encoded: Vec<u8> = bincode::serialize(&new_map).unwrap();
                        f.write_all(&encoded).expect("Failed to write file!?");
                        return DashMap::new();
                    }
                    panic!("I/O Error: {}", io_error)
                }
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

pub fn read_file_to_bytes(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read_exact(&mut buffer).expect("buffer overflow");

    buffer
}
