use proconio::input;

fn main() {
    input! {
        n: usize,
        pq: [(f64, f64); n],
    }

    let mut ans: f64 = 0.0;
    for i in 0..n {
        ans += pq[i].1 / pq[i].0;
    }

    println!("{}", ans);
}
