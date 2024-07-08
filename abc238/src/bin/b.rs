use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }

    let mut cut: Vec<u32> = vec![0; n + 2];
    for i in 0..n {
        cut[i + 1] = (cut[i] + a[i]) % 360
    }
    cut[n + 1] = 360;
    cut.sort();
    let mut ans: u32 = 0;
    for i in 1..n + 2 {
        ans = if ans < cut[i] - cut[i - 1] { cut[i] - cut[i - 1] } else { ans }
    }

    println!("{}", ans);
}
