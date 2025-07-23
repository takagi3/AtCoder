use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;

fn main() {
    input! {
        s: Chars,
    }

    if s.len() % 2 != 0 {
        println!("No");
        return;
    }
    let mut seen = HashSet::new();
    let ok = s
        .chunks(2)
        .all(|chunk| chunk[0] == chunk[1] && seen.insert(chunk[0]));

    println!("{}", if ok { "Yes" } else { "No" });
}
