use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let ans = if s.windows(2).any(|w| w[0] == w[1]) {
        "No"
    } else {
        "Yes"
    };

    println!("{}", ans);
}
