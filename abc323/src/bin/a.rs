use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let ans = if (1..16).step_by(2).all(|i| s[i] == '0') {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}
