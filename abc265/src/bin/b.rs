use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut t: i64,
        a: [i64; n - 1],
        xy: [(usize, i64); m],
    }

    let mut ans: &str = "Yes";
    let mut next: usize = 0;
    for i in 0..n - 1 {
        t -= a[i];
        if t <= 0 {
            ans = "No";
            break;
        }
        if next < m && xy[next].0 == i + 2 {
            t += xy[next].1;
            next += 1;
        }
    }

    println!("{}", ans);
}
