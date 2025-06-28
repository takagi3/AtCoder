use proconio::input;

fn main() {
    input! {
        s: [String; 12],
    }

    println!(
        "{}",
        s.iter()
            .enumerate()
            .filter(|(i, s)| s.len() == i + 1)
            .count()
    );
}
