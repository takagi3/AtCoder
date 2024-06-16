use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [u32; n],
    }

    let mut cnt: u32 = 0;
    for i in 0..n {
        'calc: for a in 1..=333 {
            for b in 1..=333 {
                if 4 * a * b + 3 * a + 3 * b == s[i] {
                    cnt += 1;
                    break 'calc;
                }
            }
        }
    }
    let ans: u32 = n as u32 - cnt;

    println!("{}", ans);
}
