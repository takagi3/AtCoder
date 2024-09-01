use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    }

    let mut ans: &str = "correct";
    'nest: for i in 0..n {
        for j in 0..n {
            if i == j { continue; }
            if a[i][j] == 'W' && a[j][i] == 'L' {
                continue;
            } else if a[i][j] == 'L' && a[j][i] == 'W' {
                continue;
            } else if a[i][j] == 'D' && a[j][i] == 'D' {
                continue;
            }
            ans = "incorrect";
            break 'nest;
        }
    }

    println!("{}", ans);
}
