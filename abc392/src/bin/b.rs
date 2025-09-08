use itertools::Itertools;
use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: u32,
        m: usize,
        a: [u32; m],
    }

    let set: HashSet<_> = a.into_iter().collect();
    let ans: Vec<_> = (1..=n).filter(|x| !set.contains(x)).collect();

    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
}
