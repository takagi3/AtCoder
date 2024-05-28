use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut ans: &str = "No";
    if s[n - 1] == 'o' {
        ans = "Yes"
    }

    println!("{}", ans);
}
