use std::cmp::max;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    let mut dp: Vec<u64> = vec![0; n];
    dp[0] = a[0];
    dp[1] = a[1];
    dp[2] = dp[0] + a[2];
    for i in 3..n {
        dp[i] = max(dp[i - 3] + a[i], dp[i - 2] + a[i]);
    }
    let ans: u64 = max(dp[n - 1], dp[n - 2]);

    println!("{}", ans);
}
