use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u32; n],
        lr: [(usize, usize); q],
    }

    let mut b: Vec<u32> = vec![0; n + 1];
    for i in 1..n + 1 {
        b[i] = a[i - 1] + b[i - 1];
    }

    for i in 0..q {
        let ans = b[lr[i].1] - b[lr[i].0 - 1];
        println!("{}", ans);
    }
}
