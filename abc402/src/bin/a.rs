use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    println!(
        "{}",
        s.into_iter()
            .filter(|&c| c.is_ascii_uppercase())
            .collect::<String>()
    );
}
