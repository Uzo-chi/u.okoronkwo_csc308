use std::fs::*;
use std::io::Write;
use std::process::Command;

fn main() {
    let fname = "output.txt";
    let result = Command::new("echo")
        .arg("Rust Process Management")
        .output()
        .unwrap()
        .stdout;
    let output = String::from_utf8_lossy(&result);
    let mut f = File::create(fname).unwrap();

    f.write_all(output.as_bytes()).unwrap();
}
