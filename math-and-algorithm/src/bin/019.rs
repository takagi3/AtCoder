use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut cnt: Vec<i64> = vec![0; 4];
    for i in 0..n {
        cnt[a[i]] += 1
    }
    let ans: i64 = ((cnt[1] * (cnt[1] - 1)) + (cnt[2] * (cnt[2] - 1)) + (cnt[3] * (cnt[3] - 1))) / 2;

    println!("{}", ans);
}
