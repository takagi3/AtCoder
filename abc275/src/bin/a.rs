use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [u32; n],
    }

    let mut ans: usize = 0;
    let mut max: u32 = 0;
    for i in 0..n {
        if max < h[i] {
            ans = i + 1;
            max = h[i];
        }
    }

    println!("{}", ans);
}
