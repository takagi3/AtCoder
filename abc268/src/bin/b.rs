use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut ans: &str = "Yes";
    if s.len() > t.len() {
        ans = "No";
    } else {
        for i in 0..s.len() {
            if s[i] != t[i] {
                ans = "No";
            }
        }
    }

    println!("{}", ans);
}
