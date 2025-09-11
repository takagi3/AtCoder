use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let b = s.as_bytes();
    let n = b.len();
    let mut ans = 0usize;
    for i in 0..n {
        let max_j = (n.saturating_sub(i) - 1) / 2;
        for j in 1..=max_j {
            if b[i] == b'A' && b[i + j] == b'B' && b[i + 2 * j] == b'C' {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
