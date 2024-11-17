use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let ans = if s.contains(&'x') {
        "No"
    } else if s.contains(&'o') {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}
