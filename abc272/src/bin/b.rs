use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        friends: [[usize]; m],
    }

    let mut pair: Vec<Vec<bool>> = vec![vec![false; n + 1]; n + 1];
    for i in 0..m {
        for &j in &friends[i] {
            for &k in &friends[i] {
                pair[j][k] = true
            }
        }
    }
    let mut ans: &str = "Yes";
    'out: for i in 1..=n {
        for j in 1..=n {
            if !pair[i][j] {
                ans = "No";
                break 'out;
            }
        }
    }

    println!("{}", ans);
}
