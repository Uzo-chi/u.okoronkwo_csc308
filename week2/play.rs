fn main() {
    println!("{}", addWithoutParams());
    println!("{}", addWithParams(2, 4));
    println!("{}", concat(String::from("twe"), "lve"));
}

fn addWithoutParams() -> i32 {
    let a = 5;
    let b = 7;
    a + b
}

fn addWithParams(x: i32, y: i32) -> i32 {
    x + y
}

fn concat(str1: String, str2: &str) -> String {
    str1 + str2
}
