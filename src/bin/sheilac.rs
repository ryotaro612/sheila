use std::os::unix::net::UnixStream;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {

      let mut stream: UnixStream = UnixStream::connect("/home/ryotaro/a.socket")?;
      let a = String::from("dあん");
       stream.write_all(a.as_bytes())?;
       stream.shutdown(std::net::Shutdown::Write)?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    println!("{response} CLIENT");
    Ok(())
}