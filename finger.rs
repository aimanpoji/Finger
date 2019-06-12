use std::io::Command;

fn main() {
    // Spawn a process, wait for it to finish, and collect it's output
    let the_output = Command::new("finger").output()
        .ok().expect("Failed to execute.");
    // Encode the resulting data.
    let encoded = String::from_utf8_lossy(the_output.output.as_slice());
    print!("{}", encoded);
}
