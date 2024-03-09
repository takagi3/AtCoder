use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    let mut dp: Vec<u64> = vec![0; n + 1];
    dp[1] = a[0];
    dp[2] = a[1];
    for i in 3..=n {
        dp[i] = max(dp[i - 2], dp[i - 3]) + a[i - 1]
    }

    println!("{}", max(dp[n], dp[n - 1]));
}
