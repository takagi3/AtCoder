use std::f64::consts::PI;
use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        d: f64,
    }

    let dist: f64 = (a.powf(2.0) + b.powf(2.0)).sqrt();
    let rad: f64 = b.atan2(a);
    let moved: f64 = d * PI / 180.0;
    let x: f64 = dist * (rad + moved).cos();
    let y: f64 = dist * (rad + moved).sin();

    println!("{} {}", x, y);
}
