use std::io;

struct Student {
    name: String,
    score: f32,
}

impl Student {
    fn new(name: String, score: f32) -> Self {
        Self { name, score }
    }

    fn check_grade(&self) {
        if self.score >= 65.0 {
            println!("Student {} passed!", self.name);
        } else {
            println!("Student {} did not pass!", self.name);
        }
    }
}

fn main() {
    println!("Enter a student's name:");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Something went wrong!");
    let name: String = name.trim().to_string();

    println!("Enter the student's score (passing grade is 65):");
    let mut score = String::new();
    io::stdin()
        .read_line(&mut score)
        .expect("Something went wrong!");
    let score: f32 = score.trim().parse().expect("Not a valid score!");

    let my_student = Student::new(name, score);

    my_student.check_grade();
}
