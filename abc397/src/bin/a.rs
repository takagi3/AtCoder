use proconio::input;

fn main() {
    input! {
        x: f32,
    }

    println!(
        "{}",
        if x >= 38.0 {
            1
        } else if 38.0 > x && x >= 37.5 {
            2
        } else {
            3
        }
    );
}
