use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    println!("{}", (s[0] as u8 - '0' as u8) * (s[2] as u8 - '0' as u8));
}
