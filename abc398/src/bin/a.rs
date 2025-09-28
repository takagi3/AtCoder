use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    println!(
        "{}",
        (0..n)
            .map(|i| {
                if i == n / 2 || (n % 2 == 0 && i == n / 2 - 1) {
                    '='
                } else {
                    '-'
                }
            })
            .collect::<String>()
    );
}
