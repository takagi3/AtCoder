use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
    }

    println!(
        "{}",
        (1..l)
            .chain((l..=r).rev())
            .chain(r + 1..=n)
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
