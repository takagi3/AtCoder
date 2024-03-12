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

    let dist_xy: f64 = ((x2 - x1).powf(2f64) + (y2 - y1).powf(2f64)).sqrt();
    let ans: u32;
    if r1.max(r2) > dist_xy + r1.min(r2) {
        ans = 1;
    } else if r1.max(r2) == dist_xy + r1.min(r2) {
        ans = 2;
    } else if dist_xy == r1 + r2 {
        ans = 4;
    } else if dist_xy > r1 + r2 {
        ans = 5;
    } else {
        ans = 3;
    }

    println!("{}", ans);
}
