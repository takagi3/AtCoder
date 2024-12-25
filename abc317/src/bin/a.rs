use proconio::input;

fn main() {
    input! {
        n: usize,
        h: u32,
        x: u32,
        p: [u32; n],
    }

    let ans = p
        .iter()
        .enumerate()
        .filter(|&(_, &pi)| h + pi >= x)
        .min_by_key(|&(_, &pi)| pi)
        .map(|(i, _)| i + 1)
        .unwrap_or(0);

    println!("{}", ans);
}
