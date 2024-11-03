use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    if let Some((i, _)) = s.iter().enumerate().find(|&(_, &c)| c.is_uppercase()) {
        println!("{}", i + 1);
    }
}
