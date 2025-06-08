use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n * (n + 1) / 2],
    }

    println!(
        "{}",
        (1..=n).fold(1, |next, i| {
            let (r, c) = (next.max(i) - 1, next.min(i) - 1);
            a[r * (r + 1) / 2 + c]
        })
    );
}
