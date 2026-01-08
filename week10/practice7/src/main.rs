use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("hello.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}
