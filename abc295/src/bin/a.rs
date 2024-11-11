use std::collections::HashSet;
use proconio::input;

fn main() {
    input! {
        n: usize,
        w: [String;n],
    }

    let words: HashSet<&str> = ["and", "not", "that", "the", "you"].iter().cloned().collect();
    let ans: &str = if w.iter().any(|word| words.contains(word.as_str())) {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}
