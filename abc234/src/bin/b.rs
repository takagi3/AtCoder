use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(f64, f64); n],
    }

    let mut ans: f64 = 0.0;
    for i in 0..n - 1 {
        for j in i + 1..n {
            if ((xy[i].0 - xy[j].0).powf(2.0) + (xy[i].1 - xy[j].1).powf(2.0)).sqrt() > ans {
                ans = ((xy[i].0 - xy[j].0).powf(2.0) + (xy[i].1 - xy[j].1).powf(2.0)).sqrt()
            }
        }
    }

    println!("{}", ans);
}
