use std::process::Command;
use std::net::(TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write, Error};

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
	println!("Incoming connecttion from: {}", stream.peer_addr()?);
	let mut buf = [0; 512];
	loop {
		let msg = stream.read(&mut buf)?;
		if msg == 0 { return Ok(())}
		stream.write(&buf[..msg])?;
	}
}



fn main() {
	let listener = TcpListener::bind("0.0.0.0:3333").except("Could not bind");
	for stream in listener.incoming() {
		match stream {
			Err(e) => { eprintln!("failed: {}", e) }
			Ok(stream) => {
				thread::spawn(move || {
					handle_client(stream.unwrap_or_else(|error| eprintln!("{:?}", error));
					});
			}
		}	
			
			
	}
	
	// Spawn a process, wait for it to finish, and collect it's output
    let mut cmd = Command::new("finger");
    cmd.arg("root");
     //execute command
    match cmd.output(){
    Ok(o) => {
        //else
        unsafe {
            let msg = String::from_utf8_unchecked(o.stdout));
        }
        
    },
    Err(e) => {
        println!("there is an error! {} ",e);
        }
}
}
