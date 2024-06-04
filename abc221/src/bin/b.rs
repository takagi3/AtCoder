use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
        t: Chars,
    }

    let mut swap: char;
    let mut ans: &str = if s == t { "Yes" } else { "No" };
    for i in 1..s.len() {
        swap = s[i - 1];
        s[i - 1] = s[i];
        s[i] = swap;
        if s == t {
            ans = "Yes";
            break;
        } else {
            swap = s[i - 1];
            s[i - 1] = s[i];
            s[i] = swap;
        }
    }

    println!("{}", ans);
}
