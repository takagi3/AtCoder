use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
        t: Chars,
    }

    let ans = s.iter()
        .zip(t.iter())
        .filter(|(a, b)| a != b)
        .count();

    println!("{}", ans);
}
