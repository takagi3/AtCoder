use proconio::input;

fn main() {
    input! {
        ax: i64,
        ay: i64,
        bx: i64,
        by: i64,
        cx: i64,
        cy: i64,
    }

    let ans: f64;
    if (ax - bx) * (cx - bx) + (ay - by) * (cy - by) <= 0 {
        ans = (((ax - bx).pow(2) + (ay - by).pow(2)) as f64).sqrt();
    } else if (ax - cx) * (bx - cx) + (ay - cy) * (by - cy) <= 0 {
        ans = (((ax - cx).pow(2) + (ay - cy).pow(2)) as f64).sqrt();
    } else {
        let area: f64 = (((ax - bx) * (cy - by) - (ay - by) * (cx - bx)) as f64).abs();
        let bc: f64 = (((bx - cx).pow(2) + (by - cy).pow(2)) as f64).sqrt();
        ans = area / bc;
    }

    println!("{}", ans);
}
