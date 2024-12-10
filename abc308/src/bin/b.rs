use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        c: [String; n],
        d: [String; m],
        p: [u32; m+1],
    }

    let price_map: HashMap<_, _> = d.iter().zip(p.iter().skip(1)).collect();
    let ans: u32 = c.iter()
        .map(|item| *price_map.get(item).unwrap_or(&&p[0]))
        .sum();

    println!("{}", ans);
}
