use std::io;

fn main() {
    let mut sentence = String::new();
    println!("Enter a sentence: ");
    io::stdin()
        .read_line(&mut sentence)
        .expect("Something went wrong!");
    let sentence = sentence.trim().to_string();

    let l_word = last_word(&sentence);
    println!("The last word is '{l_word}'");
}

fn last_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut words = Vec::new();
    let mut count = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' || i == s.len() - 1 {
            words.push(&s[count..i + 1]);
            count = i + 1;
        }
    }

    words.pop().unwrap()
}
