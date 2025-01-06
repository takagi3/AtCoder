use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let ans = s.windows(3)
        .position(|w| w == ['A', 'B', 'C'])
        .map_or(-1, |i| (i + 1) as isize);

    println!("{}", ans);
}
