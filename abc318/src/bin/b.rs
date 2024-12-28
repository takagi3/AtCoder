use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        rectangles: [(usize, usize, usize, usize); n],
    }

    let sheets: HashSet<_> = rectangles
        .iter()
        .flat_map(|&(a, b, c, d)| (a..b).flat_map(move |j| (c..d).map(move |k| (j, k))))
        .collect();

    println!("{}", sheets.len());
}
