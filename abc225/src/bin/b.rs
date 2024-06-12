use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n - 1],
    }

    let mut ans: &str = "No";
    let mut cnt: Vec<usize> = vec![0; n + 1];
    for i in 0..n - 1 {
        cnt[ab[i].0] += 1;
        cnt[ab[i].1] += 1;
    }
    for i in 0..=n {
        if cnt[i] == n - 1 {
            ans = "Yes";
            break;
        }
    }

    println!("{}", ans);
}
