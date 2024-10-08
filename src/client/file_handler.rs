use std::{fs::File, io::Read};
use std::io::Write;

pub fn read_file(path: &str) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut file = File::open(path)
        .expect(format!("FileError: Error opening file {}\n", path).as_str());

    let _ = file.read_to_end(&mut buffer)
        .expect(format!("FileError: Error reading file {}\n", path).as_str());
    
    buffer
}

pub fn write_file(buf: Vec<u8>, name: &str) {
    let mut file = File::create(name)
        .expect("Could not create audio file");

    file.write_all(buf.as_slice())
        .expect("Could not write audio file");
}