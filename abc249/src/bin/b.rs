use std::collections::HashSet;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut set: HashSet<char> = HashSet::new();
    let mut ans: &str = "Yes";
    let mut upper: bool = false;
    let mut lower: bool = false;
    for i in 0..s.len() {
        if s[i].is_uppercase() {
            upper = true;
        }
        if s[i].is_lowercase() {
            lower = true;
        }
        if set.contains(&s[i]) {
            ans = "No";
            break;
        } else {
            set.insert(s[i]);
        }
    }
    if !upper || !lower {
        ans = "No"
    }

    println!("{}", ans);
}
