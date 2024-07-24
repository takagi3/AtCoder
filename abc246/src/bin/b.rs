use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
    }

    let c: f64 = (a * a + b * b).sqrt();
    let x: f64 = a / c;
    let y: f64 = b / c;

    println!("{} {}", x, y);
}
