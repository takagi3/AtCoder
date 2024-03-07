use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    }

    // for i in 0u64..(1 << n) {
    //     let mut sum: u64 = 0;
    //     for j in 0..n {
    //         if i & (1 << j) != 0 { sum += a[j] }
    //     }
    //     if sum == s {
    //         println!("Yes");
    //         return;
    //     }
    // }

    let mut dp: Vec<Vec<bool>> = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;
    for i in 1..=n {
        for j in 0..=s {
            if j >= a[i - 1] && dp[i - 1][j - a[i - 1]] {
                dp[i][j] = true;
            } else {
                dp[i][j] = dp[i - 1][j];
            }
        }
        if dp[i][s] {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
