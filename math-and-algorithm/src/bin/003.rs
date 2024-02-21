use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    println!("{}", a.iter().sum::<u64>());
}
