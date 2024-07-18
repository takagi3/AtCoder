use std::collections::HashSet;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }

    let mut cnt1: u32 = 0;
    let mut cnt2: u32 = 0;
    let mut set: HashSet<usize> = HashSet::new();
    for i in 0..n {
        if a[i] == b[i] {
            cnt1 += 1
        }
        if set.contains(&a[i]) {
            cnt2 += 1
        } else {
            set.insert(a[i]);
        }
        if set.contains(&b[i]) {
            cnt2 += 1
        } else {
            set.insert(b[i]);
        }
    }

    println!("{}", cnt1);
    println!("{}", cnt2 - cnt1);
}
