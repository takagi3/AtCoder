use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let len: usize = s.len();
    let mut ans: &str = "";
    if s[len - 1] == 'r' {
        ans = "er"
    } else if s[len - 1] == 't' {
        ans = "ist"
    }

    println!("{}", ans);
}
