use std::process::Command;

fn main() {
    let mut ping = Command::new("ping")
        .arg("google.com")
        .spawn()
        .expect("Failed to spawn");

    let mut sleep = Command::new("sleep")
        .arg("5")
        .spawn()
        .expect("Failed to spawn");

    sleep.wait().unwrap();
    ping.kill().unwrap();
}
