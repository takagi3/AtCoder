use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let ans: char = s[s.len() / 2];

    println!("{}", ans);
}
