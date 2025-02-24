use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = String::new();
    for c in s {
        if c == '.' {
            ans.clear();
        } else {
            ans.push(c);
        }
    }

    println!("{}", ans);
}