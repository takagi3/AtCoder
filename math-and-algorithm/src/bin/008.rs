use proconio::input;

fn main() {
    input! {
        n: u32,
        s: u32,
    }

    let mut ans: u32 = 0;
    for i in 1..=n {
        for j in 1..=n {
            if i + j <= s {
                ans += 1
            }
        }
    }

    println!("{}", ans);
}
