use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [isize; n],
    }

    println!(
        "{}",
        h.iter()
            .position(|&x| x > h[0])
            .map(|i| i as i32 + 1)
            .unwrap_or(-1)
    );
}
