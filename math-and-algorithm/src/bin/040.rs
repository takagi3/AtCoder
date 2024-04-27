use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n - 1],
        m: usize,
        b: [usize; m],
    }

    let mut dict: Vec<i64> = vec![0; n];
    for i in 0..n - 1 {
        dict[i + 1] = dict[i] + a[i]
    }
    let mut ans: i64 = 0;
    for i in 1..m {
        ans += (dict[b[i] - 1] - dict[b[i - 1] - 1]).abs()
    }

    println!("{}", ans);
}
