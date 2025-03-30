use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let ans = if ('0'..='2').contains(&s[3]) {
        "Yes"
    } else if s[3] == '3' && ('0'..='4').contains(&s[4]) {
        if s[4] == '1' && s[5] == '6' {
            "No"
        } else {
            "Yes"
        }
    } else {
        "No"
    };

    println!("{}", ans);
}
