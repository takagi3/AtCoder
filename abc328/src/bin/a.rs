use proconio::input;

fn main() {
    input! {
        n: usize,
        x: u32,
        s: [u32; n],
    }

    let ans: u32 = s.into_iter().filter(|&score| score <= x).sum();

    println!("{}", ans);
}
