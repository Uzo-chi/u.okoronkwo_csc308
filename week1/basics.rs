fn main() {
    println!("{}", grade(35));
}

fn grade(score: i32) -> String {
    if score >= 90 {
        return "Excellent!".to_string();
    } else if score >= 70 {
        return "Good job!".to_string();
    } else if score >= 40 {
        return "Needs improvement!".to_string();
    } else {
        return "Needs Remedial Practice.".to_string();
    }
}
