use proconio::input;

fn main() {
    input! {
        a: [u32; 3],
    }

    println!(
        "{}",
        if a[0] * a[1] == a[2] || a[0] * a[2] == a[1] || a[1] * a[2] == a[0] {
            "Yes"
        } else {
            "No"
        }
    );
}
