use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut ans: u32 = 6;
    if s[0] == s[1] && s[1] == s[2] {
        ans = 1
    } else if s[0] == s[1] || s[1] == s[2] || s[0] == s[2] {
        ans = 3
    }

    println!("{}", ans);
}
