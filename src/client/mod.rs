mod file_handler;

use std::io::Write;
use std::net::TcpStream;
use miniz_oxide::deflate::compress_to_vec;

pub fn send(filename: &str, destination_addr: &str) {
    let file_data = file_handler::read_file(filename);
    let compressed_file_data = compress_to_vec(file_data.as_slice(), 10);

    let mut header: Vec<u8> = Vec::new();
    header.extend(filename.len().to_be_bytes());
    header.extend("FNME".as_bytes());
    header.extend(filename.as_bytes());

    let mut data: Vec<u8> = Vec::new();
    data.extend((header.len() as u32).to_be_bytes());
    data.extend("HBEG".as_bytes());
    data.extend(header);
    data.extend("HEND".as_bytes());
    data.extend(compressed_file_data.len().to_be_bytes());
    data.extend("DBEG".as_bytes());
    data.extend(compressed_file_data);
    data.extend("DEND".as_bytes());


    let mut stream = TcpStream::connect(format!("{}:3000", destination_addr).as_str())
        .expect(format!("ClientError: Couldn't connect to {}\n", destination_addr).as_str());

    stream.write_all(data.as_slice())
        .expect("ClientError: Couldn't send files to server.");

}