use std::collections::HashSet;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut set: HashSet<Vec<u32>> = HashSet::new();
    for _ in 0..n {
        input! {
            l: usize,
            a: [u32; l],
        }
        set.insert(a);
    }
    let ans: usize = set.len();

    println!("{}", ans);
}
