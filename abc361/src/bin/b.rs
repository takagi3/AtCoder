use proconio::input;

fn main() {
    input! {
        (a, b, c, d, e, f, g, h, i, j, k, l): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32)
    }

    println!(
        "{}",
        if j <= a || k <= b || l <= c || d <= g || e <= h || f <= i {
            "No"
        } else {
            "Yes"
        }
    );
}
