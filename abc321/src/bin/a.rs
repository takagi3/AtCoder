use proconio::input;

fn main() {
    input! {
        mut n: u32,
    }

    let result = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>()
        .windows(2)
        .all(|w| w[0] > w[1]);

    println!("{}", if result { "Yes" } else { "No" });
}
