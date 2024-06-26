use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut ans: &str = "Yes";
    let mut k: i32 = 0;
    for i in 0..26 {
        if s[0] as i32 - 97 == (t[0] as i32 - 97 + i) % 26 {
            k = i;
            break;
        }
    }
    for i in 1..s.len() {
        if s[i] as i32 - 97 != (t[i] as i32 - 97 + k) % 26 {
            ans = "No";
            break;
        }
    }

    println!("{}", ans);
}
