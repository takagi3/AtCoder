use proconio::input;

fn main() {
    input! {
        r: u32,
    }

    println!("{}", 100 - r % 100);
}
