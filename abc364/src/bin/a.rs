use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    println!(
        "{}",
        if s[..n - 1]
            .windows(2)
            .any(|w| w[0] == "sweet" && w[1] == "sweet")
        {
            "No"
        } else {
            "Yes"
        }
    );
}
