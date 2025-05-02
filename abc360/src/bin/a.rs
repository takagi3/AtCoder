use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    println!(
        "{}",
        if s[0] == 'R' && (s[1] == 'M' || s[2] == 'M') || s[1] == 'R' && s[2] == 'M' {
            "Yes"
        } else {
            "No"
        }
    );
}
