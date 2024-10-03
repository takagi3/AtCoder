use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut ans: i32 = -1;
    for i in 0..s.len() {
        if s[i] == 'a' {
            ans = i as i32 + 1;
        }
    }

    println!("{}", ans);
}
