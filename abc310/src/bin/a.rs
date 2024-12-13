use proconio::input;

fn main() {
    input! {
        n: usize,
        p: u32,
        q: u32,
        d: [u32; n],
    }

    let ans = d.iter().map(|&x| q + x).min().unwrap_or(p).min(p);

    println!("{}", ans);
}
