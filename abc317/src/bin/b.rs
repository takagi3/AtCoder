use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }

    let mut ans: u32 = 0;
    if let (Some(&min), Some(&max)) = (a.iter().min(), a.iter().max()) {
        let set: HashSet<_> = a.iter().cloned().collect();
        for i in min..=max {
            if !set.contains(&i) {
                ans = i;
                break;
            }
        }
    }

    println!("{}", ans);
}
