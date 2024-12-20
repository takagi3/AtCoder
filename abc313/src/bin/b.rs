use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let set: HashSet<usize> = ab.iter().map(|&(_, b)| b).collect();
    let ans = if set.len() == n - 1 {
        (1..=n)
            .find(|&i| !set.contains(&i))
            .map_or(-1, |v| v as i32)
    } else {
        -1
    };

    println!("{}", ans);
}
