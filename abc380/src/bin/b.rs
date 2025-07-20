use proconio::input;

fn main() {
    input! {
        s: String,
    }

    println!(
        "{}",
        s.split('|')
            .filter(|seg| !seg.is_empty())
            .map(|seg| seg.matches('-').count())
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
