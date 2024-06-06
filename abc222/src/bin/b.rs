use proconio::input;

fn main() {
    input! {
        n: usize,
        p: u32,
        a: [u32; n],
    }

    let mut ans: u32 = 0;
    for i in 0..n {
        if a[i] < p {
            ans += 1
        }
    }

    println!("{}", ans);
}
