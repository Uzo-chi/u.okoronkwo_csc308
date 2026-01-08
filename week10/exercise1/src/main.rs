use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    for thread_id in 1..=3 {
        let tx_clone = tx.clone();

        thread::spawn(move || {
            for i in 1..=5 {
                let message = format!("T{}: message {}", thread_id, i);
                tx_clone.send(message).unwrap();
            }
        });
    }

    drop(tx);

    for val in rx {
        println!("{}", val);
    }
}
