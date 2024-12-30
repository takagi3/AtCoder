use proconio::input;

fn main() {
    input! {
        n: u32,
    }

    let ans: String = (0..=n)
        .map(|i| {
            (1..=9)
                .find(|&j| n % j == 0 && i % (n / j) == 0)
                .map_or('-', |j| (j as u8 + b'0') as char)
        })
        .collect();

    println!("{}", ans);
}
