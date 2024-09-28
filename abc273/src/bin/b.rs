use proconio::input;

fn main() {
    input! {
        mut x: f64,
        k: f64,
    }

    for i in 1..=k as i64 {
        x /= 10f64.powf(i as f64);
        x = x.round();
        x *= 10f64.powf(i as f64);
    }
    let ans: i64 = x as i64;

    println!("{}", ans);
}
