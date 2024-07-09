use proconio::input;

fn main() {
    input! {
        h: f64,
    }

    let ans: f64 = (h * (h + 12800000.0)).sqrt();

    println!("{}", ans);
}
