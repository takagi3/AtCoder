use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;

fn main() {
    input! {
        s: Chars,
    }

    let vowels: HashSet<char> = HashSet::from(['a', 'i', 'u', 'e', 'o']);
    let ans: String = s.into_iter().filter(|&c| !vowels.contains(&c)).collect();

    println!("{}", ans);
}
