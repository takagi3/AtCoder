use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
    }

    s.sort();

    println!("{}", s.iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("")
    );
}
