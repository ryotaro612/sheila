use std::thread;

use std::io::prelude::*;
use std::os::unix::net::{UnixStream, UnixListener};

fn handle_client(mut stream: UnixStream)  {
        let mut message = String::new();
        let a = stream
        .read_to_string(&mut message);
    a.unwrap(); 
    println!("{message}");
    println!("{message}!!");
    // ...
}


fn main() -> std::io::Result<()> {
    // https://doc.rust-lang.org/std/os/unix/net/struct.UnixListener.html#method.incoming
    let listener = UnixListener::bind("/home/ryotaro/a.socket")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("new client!");
                thread::spawn(|| handle_client(stream));
            }
            Err(err) => {
                break;
            }
        }
    }
    Ok(())
}