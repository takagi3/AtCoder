use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        s: String,
    }

    let mut substrings: HashSet<&str> = HashSet::new();
    for i in 0..s.len() {
        for j in i + 1..s.len() + 1 {
            substrings.insert(&s[i..j]);
        }
    }

    println!("{}", substrings.len());
}
