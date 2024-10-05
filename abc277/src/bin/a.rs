use proconio::input;

fn main() {
    input! {
        n: usize,
        x: u32,
        p: [u32; n],
    }

    let mut ans: usize = 0;
    for i in 0..n {
        if p[i] == x {
            ans = i + 1
        }
    }

    println!("{}", ans);
}
