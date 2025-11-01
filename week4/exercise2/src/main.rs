use std::io;

struct Circle {
    radius: f32,
}

impl Circle {
    fn new(rad: f32) -> Self {
        Self { radius: rad }
    }
    fn area(&self) -> f32 {
        (22.0 / 7.0) * (self.radius).powf(2.0)
    }
    fn circumference(&self) -> f32 {
        2.0 * (22.0 / 7.0) * self.radius
    }
}

fn main() {
    let mut num = String::new();
    println!("Enter the radius of the circle:");
    io::stdin().read_line(&mut num).expect("Some error!");
    let num: f32 = num.trim().parse().expect("Enter a number!");

    let new_circle = Circle::new(num);
    let area = new_circle.area();
    let circum = new_circle.circumference();

    println!("The area of a circle of radius {num} is {area:.2}");
    println!("The circumference of a circle of radius {num} is {circum:.2}");
}
