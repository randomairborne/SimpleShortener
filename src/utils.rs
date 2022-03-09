use std::fs::{self, File};
use std::io::Read;

pub fn read_file_to_bytes(filename: &String) -> Vec<u8> {
    let f = OpenOptions::new()
        .write(true)
        .open("/path/to/file")
        .expect("File open error");

    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}
