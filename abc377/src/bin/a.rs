use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let abc = ['A', 'B', 'C'].iter().all(|&c| s.contains(&c));

    println!("{}", if abc { "Yes" } else { "No" });
}
