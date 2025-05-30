use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    println!(
        "{}",
        a.into_iter()
            .filter(|&x| x % k == 0)
            .map(|x| (x / k).to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}