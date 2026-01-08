use std::thread;

fn main() {
    let worker = thread::spawn(|| {
        for i in 1..5 {
            println!("worker: {}", i);
        }
    });

    let newt = thread::spawn(|| {
        for j in 0..20 {
            println!("j: {}", j);
        }
    });

    worker.join().unwrap();
    newt.join().unwrap();
    println!("Main thread: done.");
}
