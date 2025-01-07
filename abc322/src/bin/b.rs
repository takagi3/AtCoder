use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        t: Chars,
    }

    let ans = match () {
        _ if s == t[..n] && s == t[m - n..] => 0,
        _ if s == t[..n] => 1,
        _ if s == t[m - n..] => 2,
        _ => 3,
    };

    println!("{}", ans);
}
