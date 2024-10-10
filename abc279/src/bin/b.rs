use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut ans: &str = "No";
    if s.len() >= t.len() {
        'nest: for i in 0..=s.len() - t.len() {
            for j in 0..t.len() {
                if s[i + j] != t[j] {
                    break;
                }
                if j == t.len() - 1 {
                    ans = "Yes";
                    break 'nest;
                }
            }
        }
    }

    println!("{}", ans);
}
