use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        x: Chars,
    }

    let mut ans: &str = "Strong";
    if x[0] == x[1] && x[1] == x[2] && x[2] == x[3] {
        ans = "Weak";
    } else if (x[0] as u32 - 47) % 10 == x[1] as u32 - 48
        && (x[1] as u32 - 47) % 10 == x[2] as u32 - 48
        && (x[2] as u32 - 47) % 10 == x[3] as u32 - 48 {
        ans = "Weak";
    }

    println!("{}", ans);
}
