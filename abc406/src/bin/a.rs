use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
        d: u32,
    }

    println!(
        "{}",
        if a > c || a == c && b > d {
            "Yes"
        } else {
            "No"
        }
    );
}
