use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    for i in 1..n {
        let mut ans: u32 = 0;
        for j in 0..n {
            if i + j < n {
                if s[j] != s[i + j] {
                    ans += 1
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        println!("{}", ans);
    }
}
