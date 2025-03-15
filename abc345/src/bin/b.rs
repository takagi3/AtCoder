use proconio::input;

fn main() {
    input! {
        x: i64,
    }

    println!("{}", x / 10 + if x > 0 && x % 10 != 0 { 1 } else { 0 });
}
