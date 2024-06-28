use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        l: usize,
        r: usize,
        s: Chars,
    }

    let mut ans: String = String::new();
    for i in 0..s.len() {
        if i < l - 1 || r <= i {
            ans.push(s[i])
        } else if l - 1 <= i && i < r {
            ans.push(s[l + r - i - 2])
        }
    }

    println!("{}", ans);
}
