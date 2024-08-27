use proconio::input;

fn main() {
    input! {
        _n: u32,
        m: u32,
        x: u32,
        t: u32,
        d: u32,
    }

    let ans: u32 = if x <= m { t } else { t - ((x - m) * d) };

    println!("{}", ans);
}
