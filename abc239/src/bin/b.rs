use proconio::input;

fn main() {
    input! {
        x: i128,
    }

    let ans: i128 = if x < 0 { (x - 9) / 10 } else { x / 10 };

    println!("{}", ans);
}
