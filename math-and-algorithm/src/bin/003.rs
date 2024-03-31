use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }

    let ans: u32 = a.iter().sum();

    println!("{}", ans);
}
