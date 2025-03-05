use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    if let Some(i) = s.windows(2).position(|w| w[0] != w[1]) {
        let ans = if i == 0 {
            if s.get(2).map_or(false, |&c| c == s[1]) {
                1
            } else {
                2
            }
        } else {
            i + 2
        };
        println!("{}", ans);
    }
}
