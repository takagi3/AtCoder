use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut cnt: Vec<u64> = vec![0; n + 1];
    for i in 1..=n {
        for j in (i..=n).step_by(i) {
            cnt[j] += 1
        }
    }
    let mut ans: u64 = 0;
    for i in 1..=n {
        ans += cnt[i] * i as u64
    }

    println!("{}", ans);
}
