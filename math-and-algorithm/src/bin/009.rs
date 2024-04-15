use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    }

    let mut ans: &str = "No";
    // for i in 0u64..(1 << n) {
    //     let mut tmp_sum: usize = 0;
    //     for j in 0..n {
    //         if i & (1 << j) != 0 {
    //             tmp_sum += a[j]
    //         }
    //     }
    //     if tmp_sum == s {
    //         ans = "Yes";
    //         break;
    //     }
    // }

    let mut dp: Vec<Vec<bool>> = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;
    for i in 0..n {
        for j in 0..=s {
            dp[i + 1][j] = if j >= a[i] && dp[i][j - a[i]] {
                true
            } else {
                dp[i][j]
            }
        }
        if dp[i + 1][s] {
            ans = "Yes";
            break;
        }
    }

    println!("{}", ans);
}
