use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    println!(
        "{}",
        match s.iter().zip(&t).position(|(a, b)| a != b) {
            Some(i) => i + 1,
            None if s.len() == t.len() => 0,
            None => s.len().min(t.len()) + 1,
        }
    );
}
