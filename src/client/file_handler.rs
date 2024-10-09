use std::{fs::File, io::Read};
use std::io::Write;

pub fn read_file(path: &str) -> Result<Vec<u8>, String> {
    let mut buffer: Vec<u8> = Vec::new();
    let file_result = File::open(path);

    match file_result {
        Ok(mut file) => {
            let read_result = file.read_to_end(&mut buffer);

            if read_result.is_err() {
                return Err(format!("FileError: Error reading file {}\n", path));
            }
        },

        Err(_) => {
            return Err(format!("FileError: Error opening file {}\n", path));
        }
    }

    Ok(buffer)
}

pub fn _write_file(buf: Vec<u8>, name: &str) -> Result<(), String> {
    let file = File::create(name);

    match file {
        Ok(mut file) => {
            let write_result = file.write_all(buf.as_slice());

            if write_result.is_err() {
                return Err(String::from("FileError: Error writing file!"));
            }

            drop(file);
            Ok(())
        },
        Err(e) => Err(format!("FileError: Could not create file: {}", e))
    }
}