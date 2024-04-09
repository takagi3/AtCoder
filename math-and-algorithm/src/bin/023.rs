use proconio::input;

fn main() {
    input! {
        n: usize,
        b: [f64; n],
        r: [f64; n],
    }

    let mut ans: f64 = 0.0;
    for i in 0..n {
        ans += b[i] + r[i];
    }
    ans /= n as f64;

    println!("{}", ans);
}
