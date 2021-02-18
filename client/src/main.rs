use std::io::{stdin, BufReader, BufWriter, Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    print!("\nName: ");
    std::io::stdout().flush().unwrap();
    let mut name = String::new();
    stdin().read_line(&mut name)?;
    name.truncate(name.len() - 1);
    name.push_str(": ");
    println!("INFO: Hit enter to check for messages at any time");

    loop {
        let mut input = String::from(&name);
        print!("Message: ");
        std::io::stdout().flush().unwrap();
        stdin().read_line(&mut input)?;
        // localhost:7878
        let stream = TcpStream::connect("134.122.15.165:7878")?;
        handle_connection(stream, &input[..])?;
    }
}

fn handle_connection(stream: TcpStream, input: &str) -> std::io::Result<()> {
    let mut writer = BufWriter::new(&stream);
    let bytes = input.as_bytes();
    writer.write_all(bytes)?;
    writer.flush()?;

    let mut reader = BufReader::new(&stream);

    let mut intermediate_buffer = Vec::new();
    reader.read_to_end(&mut intermediate_buffer)?;

    let response = String::from_utf8_lossy(&intermediate_buffer);

    // do an OS clear and don't print the all messages
    print!("{}[2J", 27 as char);
    println!("{}", response);
    Ok(())
}
