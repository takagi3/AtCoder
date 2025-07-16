use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: Chars,
    }

    let [a, b, c] = [n[0], n[1], n[2]];

    println!("{}{}{} {}{}{}", b, c, a, c, a, b);
}
