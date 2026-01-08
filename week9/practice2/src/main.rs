use std::process::Command;

fn main() {
    let child = Command::new("sleep")
        .arg("5")
        .spawn()
        .expect("Failed to spawn");

    println!("Child PID: {}", child.id());
}
