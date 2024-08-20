use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; k],
        xy: [(f64, f64); n],
    }

    let mut dist: Vec<f64> = vec![1000000.0; n];
    for i in 0..k {
        for j in 0..n {
            if j == a[i] - 1 {
                dist[j] = 0.0;
                continue;
            }
            let current_dist: f64 = ((xy[j].0 - xy[a[i] - 1].0).powf(2.0) + (xy[j].1 - xy[a[i] - 1].1).powf(2.0)).sqrt();
            dist[j] = if dist[j] > current_dist { current_dist } else { dist[j] };
        }
    }
    let mut ans: f64 = 0.0;
    for i in 0..n {
        ans = if ans < dist[i] { dist[i] } else { ans };
    }

    println!("{}", ans);
}
