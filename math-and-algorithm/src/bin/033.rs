use proconio::input;

fn main() {
    input! {
        ax: f64,
        ay: f64,
        bx: f64,
        by: f64,
        cx: f64,
        cy: f64,
    }

    let ans: f64;
    if (ax - bx) * (cx - bx) + (ay - by) * (cy - by) <= 0.0 {
        ans = ((ax - bx).powf(2.0) + (ay - by).powf(2.0)).sqrt()
    } else if (ax - cx) * (bx - cx) + (ay - cy) * (by - cy) <= 0.0 {
        ans = ((ax - cx).powf(2.0) + (ay - cy).powf(2.0)).sqrt()
    } else {
        let area: f64 = ((ax - bx) * (cy - by) - (ay - by) * (cx - bx)).abs();
        let bc: f64 = ((cx - bx).powf(2.0) + (cy - by).powf(2.0)).sqrt();
        ans = area / bc;
    }

    println!("{}", ans);
}
