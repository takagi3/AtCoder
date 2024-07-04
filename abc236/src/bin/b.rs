use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; 4 * n - 1],
    }

    let mut cnt: Vec<usize> = vec![0; n + 1];
    for i in 0..4 * n - 1 {
        cnt[a[i]] += 1
    }
    let mut ans: usize = 0;
    for i in 1..=n {
        if cnt[i] == 3 {
            ans = i
        }
    }

    println!("{}", ans);
}
