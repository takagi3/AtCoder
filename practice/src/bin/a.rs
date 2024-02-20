use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
        s: String,
    }

    println!("{} {}", a + b + c, s)
}