use proconio::input;
use proconio::marker::Chars;

fn circular_distance(pair: &[char]) -> i32 {
    let diff = (pair[0] as i32 - pair[1] as i32).abs();
    diff.min(5 - diff)
}

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    println!("{}", if circular_distance(&s) == circular_distance(&t) { "Yes" } else { "No" });
}