use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        s: Chars,
    }

    let mut freq = HashMap::new();
    for ch in s {
        *freq.entry(ch).or_insert(0) += 1;
    }
    let mut count_map = HashMap::new();
    for &count in freq.values() {
        *count_map.entry(count).or_insert(0) += 1;
    }
    for &cnt in count_map.values() {
        if cnt != 2 {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
