use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = 0;
    let mut i = 0;
    while i < s.len() {
        ans += 1;
        i += if s[i] == '0' && i + 1 < s.len() && s[i + 1] == '0' {
            2
        } else {
            1
        };
    }

    println!("{}", ans);
}
