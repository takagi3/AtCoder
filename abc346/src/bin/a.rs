use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let ans: Vec<usize> = a.windows(2).map(|w| w[0] * w[1]).collect();

    println!(
        "{}",
        ans.iter()
            .map(usize::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    );
}
