use std::io::Command;

fn main() {
    // Spawn a process, wait for it to finish, and collect it's output
    let mut cmd = Command::new("finger");
    cmd.arg("root");
     //execute command
    match cmd.output(){
    ok(o) => {
        //else
        unsafe {
            println!("Output: {}",String::from_utf8_unchecked(o.stdout));
        }
        
    }
    Err(e) => {
        println!("there is an error",e);
        }
}
