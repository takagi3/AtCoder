use proconio::input;
use num::integer::gcd;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    let mut ans: u64 = a[0];
    for i in 1..n {
        ans = gcd(a[i], ans);
    }

    println!("{}", ans);
}
