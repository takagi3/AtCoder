use proconio::input;
use std::f64::consts::PI;

fn main() {
    input! {
        a: f64,
        b: f64,
        h: f64,
        m: f64,
    }

    let long: f64 = m * 6.0;
    let short: f64 = h * 30.0 + m * 0.5;
    let angle_degrees: f64 = if (long - short).abs() < 180.0 { (long - short).abs() } else { 360.0 - (long - short).abs() };
    let angle_radians: f64 = angle_degrees * (PI / 180.0);
    let ans: f64 = ((a * a + b * b) - 2.0 * a * b * angle_radians.cos()).sqrt();

    println!("{}", ans);
}
