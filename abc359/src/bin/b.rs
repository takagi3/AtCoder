use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; 2 * n],
    }

    println!("{}", a.windows(3).filter(|w| w[0] == w[2]).count());
}
