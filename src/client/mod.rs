mod file_handler;
mod gui;

use std::io::Write;
use std::net::TcpStream;
use miniz_oxide::deflate::compress_to_vec;

pub fn send(filename: &str, destination_addr: &str) {
    let file_data = file_handler::read_file(filename);
    let compressed_file_data = compress_to_vec(file_data.as_slice(), 10);
    let (address, new_filename)  = parse_address(destination_addr);
    let header: Vec<u8> = header(new_filename);
    let data: Vec<u8> = data(header, compressed_file_data);

    let mut stream = TcpStream::connect(format!("{}:3000", address).as_str())
        .expect(format!("ClientError: Couldn't connect to {}\n", address).as_str());

    stream.write_all(data.as_slice())
        .expect("ClientError: Couldn't send files to server.");

    gui::message("rcp", "File request sent!");
}

fn parse_address(address: &str) -> (&str, &str) {
    let spliited: Vec<&str> = address.split(':').collect();

    (spliited[0], spliited[1])
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