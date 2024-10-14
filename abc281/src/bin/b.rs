use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut ans: &str = "Yes";
    if s.len() != 8 {
        ans = "No"
    } else {
        for i in 0..s.len() {
            if i == 0 || i == 7 {
                if s[i] < 'A' || 'Z' < s[i] {
                    ans = "No";
                    break;
                }
            } else if i == 1 {
                if s[i] < '1' || '9' < s[i] {
                    ans = "No";
                    break;
                }
            } else {
                if s[i] < '0' || '9' < s[i] {
                    ans = "No";
                    break;
                }
            }
        }
    }

    println!("{}", ans);
}
