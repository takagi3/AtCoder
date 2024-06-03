use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    }

    let ans: u32 = 32u32.pow(a - b);

    println!("{}", ans);
}
