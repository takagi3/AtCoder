use proconio::input;
use std::cmp::max;

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
                dp[i][j] = max(dp[i - 1][j], dp[i - 1][j - wv[i - 1].0] + wv[i - 1].1)
            } else {
                dp[i][j] = dp[i - 1][j]
            }
        }
    }

    println!("{}", dp[n].iter().max().unwrap());
}
