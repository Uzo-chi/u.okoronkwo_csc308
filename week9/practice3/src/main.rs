use std::process::Command;

fn main() {
    let mut child = Command::new("sleep")
        .arg("5")
        .spawn()
        .expect("Failed to spawn");

    let status = child.wait().unwrap();
    println!("Exited with: {}", status);
}
