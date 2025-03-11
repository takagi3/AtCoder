use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = String::new();
    for (i, part) in s.split(|&c| c == '|').enumerate() {
        if i % 2 == 0 {
            ans.extend(part);
        }
    }

    println!("{}", ans);
}
