use std::os::unix::net::UnixListener;
use std::io::prelude::*;
use std::fs;
use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let verbose = args.contains(&String::from("--verbose"));

    // Remove the socket file if it already exists
    let _ = fs::remove_file("/home/ryotaro/a.socket");

    let listener = UnixListener::bind("/home/ryotaro/a.socket")?;
    if verbose {
        println!("Server listening on /home/ryotaro/a.socket");
    }
    listener.accept()?;
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = String::new();
                stream.read_to_string(&mut buffer)?;
                if verbose {
                    println!("Received: {}", buffer);
                }
                stream.write_all(b"Hello from server")?;
            }
            Err(err) => {
                eprintln!("Connection failed: {}", err);
            }
        }
    }
    Ok(())
}