use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [u32; n],
    }

    let mut ans: u32 = h[0];
    for i in 1..n {
        if ans < h[i] {
            ans = h[i]
        } else {
            break;
        }
    }

    println!("{}", ans);
}
