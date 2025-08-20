use proconio::input;

fn main() {
    input! {
        x: u32,
    }

    const TOTAL: u32 = 2025;
    let count = (1..=9)
        .flat_map(|i| (1..=9).map(move |j| i * j))
        .filter(|&v| v == x)
        .count() as u32;

    println!("{}", TOTAL - x * count);
}
