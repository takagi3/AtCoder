use proconio::input;
use std::cmp::Reverse;

fn main() {
    input! {
        n: usize,
        t: i32,
        p: usize,
        mut l: [i32; n],
    }

    l.sort_unstable_by_key(|&d| Reverse(d - t));

    println!("{}", (l[p - 1] - t).clamp(i32::MIN, 0).abs());
}
