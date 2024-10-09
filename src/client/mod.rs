mod file_handler;
mod gui;

use std::io::Write;
use std::net::TcpStream;
use miniz_oxide::deflate::compress_to_vec;

pub fn send(filename: &str, destination_addr: &str) -> Result<(), String> {
    let file_data = file_handler::read_file(filename)?;
    let compressed_file_data = compress_to_vec(file_data.as_slice(), 10);
    let (new_filename, address)  = parse_address(destination_addr);
    let header: Vec<u8> = header(new_filename);
    let data: Vec<u8> = data(header, compressed_file_data);

    match TcpStream::connect(&address) {
        Ok(mut stream) => {
            let write_result = stream.write_all(data.as_slice());
            if write_result.is_err() {
                return Err(String::from("ClientError: Couldn't send files to server."));
            }
        },
        Err(_) => {
            return Err(format!("ClientError: Couldn't connect to {}\n", &address));
        }
    }

    gui::message("rcp", "File request sent!");
    Ok(())
}

fn parse_address(address: &str) -> (&str, String) {
    let splitted: Vec<&str> = address.split('@').collect();
    let (filename, address) = (splitted[0], splitted[1]);

    if !address.contains(':') {
        let new_address = format!("{}:3000", address);
        return (filename, new_address)
    }

    (filename, String::from(address))
}
fn header(filename: &str) -> Vec<u8> {
    let mut headers = Vec::new();
    headers.extend(filename.len().to_be_bytes());
    headers.extend("FNME".as_bytes());
    headers.extend(filename.as_bytes());

    headers
}
fn data(header: Vec<u8>, compressed_file_data: Vec<u8>) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::new();
    data.extend((header.len() as u32).to_be_bytes());
    data.extend("RHDR".as_bytes());
    data.extend(header);
    data.extend(compressed_file_data.len().to_be_bytes());
    data.extend("RDAT".as_bytes());
    data.extend(compressed_file_data);

    data
}