use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
    }

    println!(
        "{}",
        if (a + 24 - b) % 24 < (c + 24 - b) % 24 {
            "No"
        } else {
            "Yes"
        }
    );
}
