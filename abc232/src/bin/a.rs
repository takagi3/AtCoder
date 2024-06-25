use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let ans: i32 = (s[0] as i32 - 48) * (s[2] as i32 - 48);

    println!("{}", ans);
}
