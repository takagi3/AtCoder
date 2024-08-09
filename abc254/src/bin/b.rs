use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans: Vec<Vec<i32>> = vec![vec![]; n];
    let mut prev_num: i32;
    for i in 0..n {
        for j in 0..=i {
            if j == 0 || j == i {
                ans[i].push(1)
            } else {
                prev_num = ans[i - 1][j - 1] + ans[i - 1][j];
                ans[i].push(prev_num);
            }
        }
        println!("{}", ans[i].iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ")
        );
    }
}
