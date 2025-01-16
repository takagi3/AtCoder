use proconio::input;

fn main() {
    input! {
        n: u32,
    }

    let ans = (n..=919)
        .find(|&i| (i / 100) * (i / 10 % 10) == i % 10)
        .unwrap_or(n);

    println!("{}", ans);
}
