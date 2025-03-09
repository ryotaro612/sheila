use std::os::unix::net::UnixListener;
use std::io::prelude::*;
use std::fs;

fn main() -> std::io::Result<()> {
    // Remove the socket file if it already exists
    let _ = fs::remove_file("/home/ryotaro/a.socket");

    let listener = UnixListener::bind("/home/ryotaro/a.socket")?;
    println!("Server listening on /home/ryotaro/a.socket");
    listener.accept()?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::os::unix::net::UnixStream;
    use std::io::Write;

    #[test]
    fn test_server_handling() {
        let socket_path = "/tmp/test.socket";

        // Start the server in a separate thread
        thread::spawn(move || {
            let _ = fs::remove_file(socket_path);
            let listener = UnixListener::bind(socket_path).unwrap();
            for stream in listener.incoming() {
                let mut stream = stream.unwrap();
                let mut buffer = String::new();
                stream.read_to_string(&mut buffer).unwrap();
                assert_eq!(buffer, "dあん");
                stream.write_all(b"Hello from server").unwrap();
            }
        });

        // Give the server a moment to start
        thread::sleep(std::time::Duration::from_millis(100));

        // Client code
        let mut stream = UnixStream::connect(socket_path).unwrap();
        let a = String::from("dあん");
        stream.write_all(a.as_bytes()).unwrap();
        stream.shutdown(std::net::Shutdown::Write).unwrap();
        let mut response = String::new();
        stream.read_to_string(&mut response).unwrap();
        assert_eq!(response, "Hello from server");
    }
}