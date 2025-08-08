use proconio::input;

fn main() {
    input! {
        (a, b, c): (u32, u32, u32),
    }

    println!(
        "{}",
        if a == b && b == c || a + b == c || a + c == b || b + c == a {
            "Yes"
        } else {
            "No"
        }
    );
}
