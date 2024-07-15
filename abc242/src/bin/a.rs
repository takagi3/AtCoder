use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        c: f64,
        x: f64,
    }

    let ans: f64;
    if x <= a {
        ans = 1.0
    } else if x <= b {
        ans = c / (b - a)
    } else {
        ans = 0.0
    }

    println!("{}", ans);
}
