use std::collections::HashSet;
use proconio::input;

fn main() {
    input! {
        num: [u32; 5],
    }

    let mut set: HashSet<u32> = HashSet::new();
    for i in 0..5 {
        set.insert(num[i]);
    }

    println!("{}", set.len());
}
