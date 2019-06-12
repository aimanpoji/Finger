use std::process::Command;   
use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

fn main() {
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");

            let msg = b"Finger Information from serverclear";

            stream.write(msg).unwrap();
            println!("Finger Information");

            let mut data = [0 as u8; 18];
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        println!(" ");
                    } else {
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
    println!(" ");
	handle_server();
}

fn handle_server() {

let mut cmd = Command::new("finger");
    cmd.arg("root");
    match cmd.output(){
        Ok(o) => {
            unsafe {
                println!("Output: {}", String::from_utf8_unchecked(o.stdout));
                }
            },
        Err(e) => {
            println!("there is an error! {} ",e);
        }
    }
}
