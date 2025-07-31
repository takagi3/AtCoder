use proconio::input;

fn main() {
    input! {
        n: usize,
        tv: [(u32, u32); n],
    }

    println!(
        "{}",
        tv.windows(2).fold(tv[0].1, |acc, pair| {
            let dt = pair[1].0 - pair[0].0;
            acc.saturating_sub(dt) + pair[1].1
        })
    );
}
