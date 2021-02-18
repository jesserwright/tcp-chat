use std::env;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpListener;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let host = env::var("HOST").expect("Failed to read host env var");
    let port = env::var("PORT").expect("Failed to read port env var");
    let listener = TcpListener::bind(format!("{}:{}", host, port))?;

    let mut messages = String::new();

    for stream in listener.incoming() {
        let stream = stream?;
        handle_client(stream, &mut messages)?;
    }
    Ok(())
}

fn handle_client(stream: TcpStream, messages: &mut String) -> std::io::Result<()> {
    let mut reader = BufReader::new(&stream);
    let mut msg = String::new();
    reader.read_line(&mut msg)?;

    // This is an empty message (when just "enter" is hit)
    if !msg.contains(": \n") {
        messages.push_str(&msg);
    }

    let mut writer = BufWriter::new(&stream);
    writer.write_all(messages.as_bytes())?;
    writer.flush()?;
    Ok(())
}
