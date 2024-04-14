use std::cmp::max;
use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, i64); n],
    }

    let mut dp: Vec<Vec<i64>> = vec![vec![i64::MIN; w + 1]; n + 1];
    dp[0][0] = 0;
    for i in 1..=n {
        for j in 0..=w {
            if j >= wv[i - 1].0 {
                dp[i][j] = max(dp[i - 1][j - wv[i - 1].0] + wv[i - 1].1, dp[i - 1][j])
            } else {
                dp[i][j] = dp[i - 1][j]
            }
        }
    }

    let mut ans: i64 = 0;
    for i in 0..=w {
        ans = max(dp[n][i], ans)
    }

    println!("{}", ans);
}
