use proconio::input;

fn main() {
    input! {
        r: u32,
        x: u32,
    }

    println!(
        "{}",
        if x == 1 && 1600 <= r && r < 3000 || x == 2 && 1200 <= r && r < 2400 {
            "Yes"
        } else {
            "No"
        }
    );
}
