use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
pub fn read_bincode(filename: &String) -> dashmap::DashMap<String, String> {
    let mut f = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(filename)
        .expect("Failed to open file");

    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    let deserialized: dashmap::DashMap<String, String> = match bincode::deserialize(&buffer[..]) {
        Ok(dashmap) => dashmap,
        Err(e) => {
            match *e {
                bincode::ErrorKind::Io(ioerror) => {
                    if ioerror.kind() == io::ErrorKind::UnexpectedEof {
                        println!("Creating new database...");
                        let new_dashmap: dashmap::DashMap<String, String> = dashmap::DashMap::new();
                        let encoded: Vec<u8> = bincode::serialize(&new_dashmap).unwrap();
                        f.write(&encoded);
                        return dashmap::DashMap::new();
                    }
                    panic!("I/O Error: {}", ioerror)
                }
                _ => panic!("Error deserializing database: {}", e),
            };
        }
    };
    return deserialized;
}

pub fn save_bincode(filename: &String, map: dashmap::DashMap<String, String>) {}

pub fn read_file_to_bytes(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}
