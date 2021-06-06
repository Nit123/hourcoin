use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

fn main() {
    match TcpStream::connect("localhost:4776") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 4776");

            let msg = b"Hello!";

            stream.write(msg).unwrap();
            println!("Sent Hello, awaiting reply...");

            let mut data = [0 as u8; 6];
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        println!("Reply is ok!: {}", String::from_utf8_lossy(&data));
                    }
                    else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
                },

                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        }, 

        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }

    println!("Teriminated.");
}