use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let bytes = s.as_bytes();
    let positions: Vec<usize> = (b'A'..=b'Z')
        .map(|c| bytes.iter().position(|&b| b == c).unwrap())
        .collect();

    println!(
        "{}",
        positions
            .windows(2)
            .map(|w| w[0].abs_diff(w[1]))
            .sum::<usize>()
    );
}
