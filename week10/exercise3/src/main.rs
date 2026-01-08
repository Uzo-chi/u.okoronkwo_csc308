use chrono::Local;
use std::io::{BufRead, BufReader, Write};
use std::{fs, io};

fn main() {
    let fname = "notes.txt";
    let mut choice = String::new();
    let input = io::stdin();

    let mut file: fs::File;

    loop {
        choice.clear();
        println!(
            "\nWhat do you want to do? Enter:\n - 'a' to append to the file\n - 'v' to view all notes\n - 'e' to exit\n\nChoice:"
        );

        input.read_line(&mut choice).expect("Could not read value");
        let choice = choice.trim();

        if choice == "a" {
            file = fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(fname)
                .unwrap();

            let mut text = String::new();

            println!("Enter text continuously (enter 'exit!' on a new line to quit):");

            loop {
                text.clear();

                input.read_line(&mut text).expect("Failed to read line!");

                let text_trim = text.trim();

                if text_trim == "exit!" {
                    println!("\nExiting program!");
                    break;
                }

                let now = Local::now();

                writeln!(file, "{} ----- [{}]", text_trim, now).unwrap();
            }
        } else if choice == "v" {
            file = fs::File::open(fname).unwrap();
            let reader = BufReader::new(file);

            for line in reader.lines() {
                println!("{}", line.unwrap());
            }
        } else if choice == "e" {
            println!("Exiting program!");
            break;
        } else {
            println!("Invalid input!");
            continue;
        }
    }
}
