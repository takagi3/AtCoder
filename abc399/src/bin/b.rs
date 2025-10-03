use proconio::input;
use std::cmp::Reverse;

fn main() {
    input! {
        n: usize,
        p: [u32; n],
    }

    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_unstable_by_key(|&i| Reverse(p[i]));

    let mut ans = vec![0u32; n];
    let mut prev: Option<u32> = None;
    let mut rank = 0usize;

    for (pos, &i) in idx.iter().enumerate() {
        if prev.map_or(true, |v| p[i] != v) {
            rank = pos + 1;
            prev = Some(p[i]);
        }
        ans[i] = rank as u32;
    }

    for a in ans {
        println!("{}", a);
    }
}
