use std::collections::HashSet;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }

    let mut cnt: HashSet<u32> = HashSet::new();
    for i in 0..n {
        cnt.insert(a[i]);
    }
    let ans: usize = cnt.len();

    println!("{}", ans);
}
