use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    println!(
        "{}",
        if a.windows(3).any(|w| w[0] == w[1] && w[1] == w[2]) {
            "Yes"
        } else {
            "No"
        }
    );
}
