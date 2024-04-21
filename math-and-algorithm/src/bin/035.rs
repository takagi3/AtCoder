use proconio::input;

fn main() {
    input! {
        x1: f64,
        y1: f64,
        r1: f64,
        x2: f64,
        y2: f64,
        r2: f64,
    }

    let ans: u32;
    let xy_dist: f64 = ((x1 - x2).powf(2.0) + (y1 - y2).powf(2.0)).sqrt();
    if xy_dist + r1.min(r2) < r1.max(r2) {
        ans = 1;
    } else if xy_dist + r1.min(r2) == r1.max(r2) {
        ans = 2;
    } else if xy_dist == r1 + r2 {
        ans = 4;
    } else if xy_dist > r1 + r2 {
        ans = 5;
    } else {
        ans = 3;
    }

    println!("{}", ans);
}
