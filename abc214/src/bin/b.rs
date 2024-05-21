use proconio::input;

fn main() {
    input! {
        s: u32,
        t: u32,
    }

    let mut ans: u32 = 0;
    for a in 0..=s {
        for b in 0..=s {
            for c in 0..=s {
                if a + b + c <= s && a * b * c <= t {
                    ans += 1
                }
            }
        }
    }

    println!("{}", ans);
}
