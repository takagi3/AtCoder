use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut cnt: Vec<u64> = vec![0; 100000];
    for i in 0..n {
        cnt[a[i]] += 1;
    }

    let mut ans: u64 = 0;
    for i in 1..50000 {
        ans += cnt[i] * cnt[100000 - i];
    }
    if cnt[50000] > 0 {
        ans += cnt[50000] * (cnt[50000] - 1) / 2;
    }

    println!("{}", ans);
}
