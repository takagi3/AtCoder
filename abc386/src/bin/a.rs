use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        cards: [u32; 4],
    }

    let kind: HashSet<u32> = cards.into_iter().collect();

    println!("{}", if kind.len() == 2 { "Yes" } else { "No" });
}
