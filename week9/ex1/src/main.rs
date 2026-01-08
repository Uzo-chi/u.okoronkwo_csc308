use std::process::Command;

fn main() {
    let mut child1 = Command::new("sleep")
        .arg("5")
        .spawn()
        .expect("Failed to spawn");

    let mut child2 = Command::new("ls")
        .arg("-la")
        .spawn()
        .expect("Failed to spawn!");

    let mut child3 = Command::new("echo")
        .arg("Hello from child")
        .output()
        .unwrap();

    println!("stdout: {}", String::from_utf8_lossy(&child3.stdout));

    child2.wait().expect("Failed to wait for ls");
    child1.wait().expect("Failed to wait for sleep");
}
