use std::collections::HashSet;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    let mut set: HashSet<i32> = HashSet::new();
    for i in 0..9 {
        set.insert(s[i] as i32 - 48);
    }
    let mut ans: i32 = 0;
    for i in 0..=9 {
        if !set.contains(&i) {
            ans = i;
            break;
        }
    }

    println!("{}", ans);
}
