use std::collections::HashSet;
use proconio::input;

fn main() {
    input! {
        n: usize,
        w: u32,
        a: [u32; n],
    }

    let mut set: HashSet<u32> = HashSet::new();
    for i in 0..n {
        if a[i] <= w {
            set.insert(a[i]);
        }
    }
    if n >= 2 {
        for i in 0..n - 1 {
            for j in i + 1..n {
                if a[i] + a[j] <= w {
                    set.insert(a[i] + a[j]);
                }
            }
        }
    }
    if n >= 3 {
        for i in 0..n - 2 {
            for j in i + 1..n - 1 {
                for k in j + 1..n {
                    if a[i] + a[j] + a[k] <= w {
                        set.insert(a[i] + a[j] + a[k]);
                    }
                }
            }
        }
    }
    let ans: usize = set.len();

    println!("{}", ans);
}
