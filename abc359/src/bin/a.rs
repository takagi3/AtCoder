use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    println!(
        "{}",
        s.into_iter().filter(|name| name == "Takahashi").count()
    );
}
