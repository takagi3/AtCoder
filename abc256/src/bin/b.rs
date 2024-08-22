use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }

    let mut b: Vec<u32> = vec![0; n];
    for i in 0..n {
        for j in 0..n {
            if i < j {
                continue;
            }
            b[j] += a[i]
        }
    }
    let mut ans: u32 = 0;
    for i in 0..n {
        if b[i] >= 4 {
            ans += 1
        }
    }

    println!("{}", ans);
}
