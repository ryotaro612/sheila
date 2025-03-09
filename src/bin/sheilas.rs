use std::os::unix::net::UnixListener;
use std::io::prelude::*;
use std::fs;

fn main() -> std::io::Result<()> {
    // Remove the socket file if it already exists
    let _ = fs::remove_file("/home/ryotaro/a.socket");

    let listener = UnixListener::bind("/home/ryotaro/a.socket")?;
    println!("Server listening on /home/ryotaro/a.socket");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = String::new();
                stream.read_to_string(&mut buffer)?;
                println!("Received: {}", buffer);
                stream.write_all(b"Hello from server")?;
            }
            Err(err) => {
                eprintln!("Connection failed: {}", err);
            }
        }
    }
    Ok(())
}