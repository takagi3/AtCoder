use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let ans: String;
    if s[0] == s[1] && s[1] == s[2] {
        ans = String::from("-1")
    } else {
        if s[0] == s[1] {
            ans = String::from(s[2])
        } else if s[0] == s[2] {
            ans = String::from(s[1])
        } else {
            ans = String::from(s[0])
        }
    }

    println!("{}", ans);
}
