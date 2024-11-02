use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [u32; n],
        b: [usize; m],
    }

    let ans: u32 = b.iter().map(|&i| a[i - 1]).sum();

    println!("{}", ans);
}
