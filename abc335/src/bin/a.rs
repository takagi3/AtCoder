use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
    }

    let s_length = s.len();
    s[s_length - 1] = '4';

    println!(
        "{}",
        s.iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("")
    );
}
