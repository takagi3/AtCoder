use proconio::input;

fn main() {
    input! {
        y: u32,
    }

    let ans: u32 = (y + 1) / 4 * 4 + 2;

    println!("{}", ans);
}
