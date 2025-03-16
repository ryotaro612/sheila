use std::io::prelude::*;
use std::os::unix::net::UnixStream;

fn main() -> std::io::Result<()> {
    let mut stream: UnixStream = UnixStream::connect("/home/ryotaro/a.socket")?;
    stream.write_all(b"hello world")?;
    stream.flush()?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    println!("{response}");
    Ok(())
}
