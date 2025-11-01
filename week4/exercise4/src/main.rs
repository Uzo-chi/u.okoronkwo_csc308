use std::io;

struct SentenceAnalyzer {
    sentence: String,
    longest: String,
    shortest: String,
}

impl SentenceAnalyzer {
    fn new(sentence: String) -> Self {
        Self {
            sentence,
            longest: String::new(),
            shortest: String::new(),
        }
    }

    fn analyze(&mut self) {
        let words: Vec<&str> = self
            .sentence
            .split_whitespace()
            .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()))
            .filter(|w| !w.is_empty())
            .collect();

        if words.is_empty() {
            println!("No valid words found!");
        }

        self.longest = words[0].to_string();
        self.shortest = words[0].to_string();

        for word in &words {
            if word.len() > self.longest.len() {
                self.longest = word.to_string();
            }
            if word.len() < self.shortest.len() {
                self.shortest = word.to_string();
            }
        }
    }
}

fn main() {
    let mut sentence = String::new();
    println!("Enter a sentence:");
    io::stdin()
        .read_line(&mut sentence)
        .expect("Something went wrong!");
    let sentence: String = sentence.trim().to_string();

    let mut my_sentence = SentenceAnalyzer::new(sentence);
    my_sentence.analyze();

    println!(
        "\nThe longest word is '{}'\nThe shortest word is '{}'",
        my_sentence.longest, my_sentence.shortest
    );
}
