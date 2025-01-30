use proconio::input;

fn main() {
    input! {
        n: u32,
        s: u32,
        m: u32,
        l: u32,
    }

    let mut ans = u32::MAX;
    for i in 0..=((n + 5) / 6) {
        for j in 0..=((n + 7) / 8) {
            let remaining = n.saturating_sub(i * 6 + j * 8);
            let k = (remaining + 11) / 12;
            ans = ans.min(s * i + m * j + l * k);
        }
    }

    println!("{}", ans);
}
