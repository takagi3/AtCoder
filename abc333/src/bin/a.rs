use proconio::input;

fn main() {
    input! {
        n: u32,
    }

    let ans: u32 = (0..n).map(|i| n * 10_u32.pow(i)).sum();

    println!("{}", ans);
}