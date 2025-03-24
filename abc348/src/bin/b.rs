use proconio::input;
use std::cmp::Reverse;

fn main() {
    input! {
        n: usize,
        xy: [(isize, isize); n],
    }

    let ans: Vec<usize> = (0..n)
        .map(|i| {
            xy.iter()
                .enumerate()
                .filter(|&(j, _)| i != j)
                .max_by_key(|&(j, &(xj, yj))| {
                    let d = (xy[i].0 - xj).pow(2) + (xy[i].1 - yj).pow(2);
                    (d, Reverse(j))
                })
                .map(|(j, _)| j + 1)
                .unwrap()
        })
        .collect();

    println!(
        "{}",
        ans.iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    );
}
