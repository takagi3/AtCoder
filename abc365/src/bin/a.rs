use proconio::input;

fn main() {
    input! {
        y: u32
    }

    println!(
        "{}",
        365 + (y % 4 == 0 && (y % 100 != 0 || y % 400 == 0)) as u32
    );
}
