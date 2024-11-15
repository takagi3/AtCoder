use proconio::input;

fn main() {
    input! {
        n: usize,
        d: i32,
        t:[i32; n],
    }

    let ans = t.windows(2)
        .find(|w| w[1] - w[0] <= d)
        .map(|w| w[1])
        .unwrap_or(-1);

    println!("{}", ans);
}
