use std::io;

fn main() {
    let mut num = String::new();
    println!("Enter a number: ");
    io::stdin().read_line(&mut num).expect("Cannot read!");
    let num: u32 = num.trim().parse().expect("Please enter a real number!");

    let factorial = |x: u32| {
        let mut res;
        if x == 0 || x == 1 {
            res = 1;
        } else {
            res = 1;
            for n in 1..=x {
                res *= n;
            }
        }
        res
    };

    println!("The factorial is: {}", factorial(num));
}
