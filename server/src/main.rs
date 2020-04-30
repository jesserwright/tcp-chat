use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let port = "localhost:7878";
    let listener = TcpListener::bind(port).expect("Failed to bind");

    let mut messages = String::new();

    for stream in listener.incoming() {
        let stream = stream.expect("Unable to accept");
        handle_client(stream, &mut messages);
    }
}

fn handle_client(stream: TcpStream, messages: &mut String) {
    let mut reader = BufReader::new(&stream);
    println!("Reading client message");
    reader.read_line(messages).expect("Could not read");
    println!("{}", &messages);

    let mut writer = BufWriter::new(&stream);
    println!("Sending to client");
    let mut response = messages.clone();
    response.push_str("#");
    writer
        .write_all(response.as_bytes())
        .expect("Could not write");
    writer.flush().expect("Could not flush");
}
