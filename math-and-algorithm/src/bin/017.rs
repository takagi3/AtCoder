use proconio::input;
use num::integer::lcm;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    let mut ans: u64 = a[0];
    for i in 1..n {
        ans = lcm(a[i], ans);
    }

    println!("{}", ans);
}
