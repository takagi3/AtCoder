use proconio::input;

fn main() {
    input! {
        s: u32,
    }

    let ans: u32 = s / 10;

    println!("{:0>4}", ans);
}
