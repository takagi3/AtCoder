use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let mut ans: usize = 0;
    let mut seen: HashSet<char> = HashSet::new();
    for (i, &ch) in s.iter().enumerate() {
        seen.insert(ch);
        if seen.len() == 3 {
            ans = i + 1;
            break;
        }
    }

    println!("{}", ans);
}
