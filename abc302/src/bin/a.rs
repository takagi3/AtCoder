use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
    }

    println!("{}", (a + b - 1) / b);
}
