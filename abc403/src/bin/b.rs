use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        t: Chars,
        u: Chars,
    }

    let ans = t
        .windows(u.len())
        .any(|w| w.iter().zip(&u).all(|(&tc, &uc)| tc == '?' || tc == uc));

    println!("{}", if ans { "Yes" } else { "No" });
}
