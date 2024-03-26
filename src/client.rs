use std::io::{self, Read, Write};
use std::net::TcpStream;

fn main() {
    let mut stream = match TcpStream::connect("127.0.0.1:7878") {
        Ok(stream) => {
            println!("Successfully connected to the server at 127.0.0.1:7878");
            stream
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
            return;
        }
    };

    loop {
        println!("Enter command (or type 'exit' to quit):");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Trim newline and check for 'exit' command
        let input = input.trim();
        if input.eq_ignore_ascii_case("exit") {
            println!("Exiting client.");
            break;
        }

        if let Ok(_) = stream.write_all(input.as_bytes()) {
            // Reading response
            let mut buffer = vec![0; 1024];
            match stream.read(&mut buffer) {
                Ok(_) => {
                    println!("Server response: {}", String::from_utf8_lossy(&buffer));
                }
                Err(e) => println!("Failed to receive response: {}", e),
            }
        }
    }
}
