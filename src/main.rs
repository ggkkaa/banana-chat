use std::io::{self, Write, Read, Error};
use std::net::TcpStream;
use std::thread;

fn main() -> Result<(), Error> {
    let mut stream = TcpStream::connect("88.193.139.141:6969").unwrap();
    println!("Connected to the server!");


    let mut out_stream = stream.try_clone().unwrap();
    let _read_thread = thread::spawn(move || {
        let mut buffer = [0; 1024];
        loop {
            match out_stream.read(&mut buffer) {
                Ok(0) => {
                    println!("Server closed the connection.");
                    break;
                }
                Ok(n) => {
                    let message = String::from_utf8_lossy(&buffer[..n]);
                    print!("{}", message);
                    io::stdout().flush().unwrap();
                }
                Err(e) => {
                    eprintln!("Error reading from server: {}", e);
                    break;
                }
                
            }
        }
    });


    let mut input = String::new();

    loop {
        io::stdin().read_line(&mut input).unwrap();
        stream.write_all(input.as_bytes()).unwrap();
        input.clear();
    }
}
