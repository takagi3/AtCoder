use proconio::input;

fn main() {
    input! {
        n: i32,
        m: i32,
        p: i32,
    }

    println!("{}", if n - m >= 0 { (n - m) / p + 1 } else { 0 });
}
