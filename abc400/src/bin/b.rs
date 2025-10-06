use proconio::input;

fn main() {
    input! {
        n: usize,
        m: u32,
    }

    let mut ans = 0usize;
    let mut pow = 1usize;
    for _ in 0..=m {
        ans += pow;
        if ans > 1_000_000_000 {
            println!("inf");
            return;
        }
        pow *= n;
    }

    println!("{}", ans);
}
