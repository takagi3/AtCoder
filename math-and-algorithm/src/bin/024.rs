use proconio::input;

fn main() {
    input! {
        n: usize,
        pq: [(f64, f64); n],
    }

    let mut ans: f64 = 0.0;
    for (p, q) in pq {
        ans += q / p;
    }

    println!("{}", ans);
}
