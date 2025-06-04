use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    }

    println!(
        "{}",
        if a == b {
            1
        } else if (a + b) % 2 != 0 {
            2
        } else {
            3
        }
    );
}
