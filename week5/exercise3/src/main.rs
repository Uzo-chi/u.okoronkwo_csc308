use ::std::process::Command;

fn main() {
    let comm = Command::new("echo")
        .arg("Hello from child process!")
        .output()
        .expect("Something went wrong!");

    println!("Status: {:?}", comm.status);
    println!("Output: {}", String::from_utf8_lossy(&comm.stdout));
}
