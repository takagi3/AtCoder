use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        a: usize,
        b: usize,
    }

    let mut ans: String = String::new();
    for i in 0..s.len() {
        if i == a - 1 {
            ans.push(s[b - 1])
        } else if i == b - 1 {
            ans.push(s[a - 1])
        } else {
            ans.push(s[i])
        }
    }

    println!("{}", ans);
}
