use std::io::{stdin, BufRead, BufReader, BufWriter, Write};
use std::net::TcpStream;

fn main() {
    let port = "localhost:7878";
    loop {
        println!("Please input message:");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read input");
        println!("Input is read");
        let stream = TcpStream::connect(port).expect("Could not connect");
        handle_connection(stream, &input[..]);
    }
}

fn handle_connection(stream: TcpStream, input: &str) {
    let mut writer = BufWriter::new(&stream);
    println!("Sending message to server");
    let bytes = input.as_bytes();
    writer.write_all(bytes).expect("Could not write");
    writer.flush().expect("Could not flush");

    let mut reader = BufReader::new(&stream);
    let mut response = Vec::new();
    println!("Reading server response");
    reader
        .read_until(b"#"[0], &mut response)
        .expect("Could not read");

    println!("ALL MESSAGES:\n{}", String::from_utf8_lossy(&response));
}
