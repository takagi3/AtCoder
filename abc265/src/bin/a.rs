use proconio::input;

fn main() {
    input! {
        x: u32,
        y: u32,
        n: u32,
    }

    let ans: u32 = if x * 3 < y { x * n } else { n / 3 * y + n % 3 * x };

    println!("{}", ans);
}
