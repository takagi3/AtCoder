use proconio::input;

fn main() {
    input! {
        n: u32,
    }

    let ans: u64 = 2u64.pow(n);

    println!("{}", ans);
}
