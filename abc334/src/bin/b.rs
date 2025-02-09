use proconio::input;

fn main() {
    input! {
        a: i64,
        m: i64,
        l: i64,
        r: i64,
    }

    let k_min = (l - a + (m - 1)).div_euclid(m);
    let k_max = (r - a).div_euclid(m);
    let ans = if k_max >= k_min { k_max - k_min + 1 } else { 0 };

    println!("{}", ans);
}