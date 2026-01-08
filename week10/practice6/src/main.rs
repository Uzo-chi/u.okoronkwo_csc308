use std::fs::File;

fn main() {
    let file = File::open("hello.txt");
    match file {
        Ok(_) => println!("Opened successfully"),
        Err(e) => println!("Error: {}", e),
    }
}
