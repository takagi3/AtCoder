use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        d: usize,
    }

    println!(
        "{}",
        (a..=b)
            .step_by(d)
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}