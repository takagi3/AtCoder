use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[u32; w]; h],
    }

    let mut ans: Vec<Vec<u32>> = vec![vec![0; h]; w];
    for i in 0..h {
        for j in 0..w {
            ans[j][i] = a[i][j]
        }
    }

    for i in 0..w {
        println!("{}", ans[i].iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ")
        );
    }
}
