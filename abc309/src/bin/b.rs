use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    }

    let mut ans = vec![vec![' '; n]; n];
    for i in 0..n {
        for j in 0..n {
            ans[i][j] = if i == 0 && j > 0 {
                a[i][j - 1]
            } else if i == n - 1 && j < n - 1 {
                a[i][j + 1]
            } else if j == 0 && i < n - 1 {
                a[i + 1][j]
            } else if j == n - 1 && i > 0 {
                a[i - 1][j]
            } else {
                a[i][j]
            };
        }
    }

    for row in ans {
        println!("{}", row.iter().collect::<String>());
    }
}
