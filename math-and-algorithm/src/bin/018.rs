use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut cnt: Vec<u64> = vec![0; 5];
    for i in 0..n {
        cnt[a[i] / 100] += 1;
    }

    println!("{}", cnt[1] * cnt[4] + cnt[2] * cnt[3]);
}
