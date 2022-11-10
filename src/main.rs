use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 512];
    stream.read(&mut buffer)?;

    let message = std::str::from_utf8(&buffer)
        .unwrap()
        .split("\r\n")
        .next()
        .unwrap();

    if let Some(response) = process_message(&message) {
        dbg!(&response);
        stream.write(response.as_bytes())?;
    }

    handle_client(stream)
}

fn process_message(message: &str) -> Option<String> {
    dbg!(&message);

    let command = message.split(" ").next().unwrap();

    match command {
        "CAP" => Some("CAP * LS :\r\n".into()),
        "USER" => Some(":localhost 001 jasondew :Welcome! Are we IRC yet? jasondew!jasondew@localhost\r\n".into()),
        "NICK" => None,
            _ => None
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6666")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?)?;
    }

    Ok(())
}
