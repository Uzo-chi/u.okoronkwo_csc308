use std::io;

struct BankAccount {
    balance: usize,
}

impl BankAccount {
    fn new(balance: usize) -> Self {
        Self { balance }
    }

    fn deposit(&mut self, amount: usize) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: usize) {
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!("You have ${} in your account.", self.balance);
    }
}

fn main() {
    let mut my_account = BankAccount::new(100000);

    loop {
        let mut choice = String::new();
        println!("\nHello user!");
        println!(
            "Would you like to:\n1. Deposit (type 1)\n2. Withdraw (type 2)\n3. Check balance (type 3)\n4. Quit (type 4)\n"
        );

        io::stdin()
            .read_line(&mut choice)
            .expect("Could not take input!");
        let choice: usize = choice.trim().parse().expect("Not a number!");

        let mut amount = String::new();

        if choice == 1 {
            println!("How much would you like to deposit: ");
            io::stdin()
                .read_line(&mut amount)
                .expect("Something went wrong!");
            let amount: usize = amount.trim().parse().expect("Enter a positive integer!");

            println!("Successfully deposited!");
            my_account.deposit(amount);
        } else if choice == 2 {
            println!("How much would you like to withdraw: ");
            io::stdin()
                .read_line(&mut amount)
                .expect("Something went wrong!");
            let amount: usize = amount.trim().parse().expect("Enter a positive integer!");

            println!("Successfully withdrawn!");
            my_account.withdraw(amount);
        } else if choice == 3 {
            my_account.check_balance();
        } else if choice == 4 {
            break;
        } else {
            println!("Invalid input!");
            continue;
        }
    }
}
