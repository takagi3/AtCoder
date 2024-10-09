use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut ans: u32 = 0;
    for i in 0..s.len() {
        if s[i] == 'v' {
            ans += 1
        } else if s[i] == 'w' {
            ans += 2
        }
    }

    println!("{}", ans);
}
