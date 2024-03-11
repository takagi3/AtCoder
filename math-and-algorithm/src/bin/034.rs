use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }

    let mut ans: f64 = f64::MAX;
    for i in 0..n - 1 {
        for j in i + 1..n {
            ans = ans.min((((xy[i].0 - xy[j].0).pow(2) + (xy[i].1 - xy[j].1).pow(2)) as f64).sqrt());
        }
    }

    println!("{}", ans);
}
