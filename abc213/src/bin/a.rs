use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    }

    let ans: u32 = a ^ b;
    println!("{}", ans);
}
