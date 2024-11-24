use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }

    let ans: Vec<u32> = a
        .windows(2)
        .flat_map(|w| {
            if w[0] < w[1] {
                (w[0]..w[1]).collect::<Vec<_>>()
            } else {
                (w[1] + 1..=w[0]).rev().collect::<Vec<_>>()
            }
        })
        .chain(std::iter::once(*a.last().unwrap()))
        .collect();

    println!(
        "{}",
        ans.iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    );
}
