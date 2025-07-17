use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        k: usize,
        s: Chars,
    }

    let ans = s
        .split(|&c| c == 'X')
        .map(|seg| seg.len() / k)
        .sum::<usize>();

    println!("{}", ans);
}
