use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    println!(
        "{}",
        s.windows(2).filter(|w| w[0] == w[1]).count()
            + ((s[0] == 'o') as usize + (s[s.len() - 1] == 'i') as usize)
    );
}
