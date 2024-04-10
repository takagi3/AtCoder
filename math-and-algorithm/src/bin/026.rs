use proconio::input;

fn main() {
    input! {
        n: u32,
    }

    let mut ans: f64 = 0.0;
    for i in 1..=n {
        ans += n as f64 / i as f64
    }

    println!("{}", ans);
}
