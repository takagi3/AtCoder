use std::collections::HashSet;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }

    let mut set: HashSet<u32> = HashSet::new();
    for i in 0..n {
        set.insert(a[i]);
    }
    let mut ans: u32 = 0;
    for i in 0..=2000 {
        if !set.contains(&i) {
            ans = i;
            break;
        }
    }

    println!("{}", ans);
}
