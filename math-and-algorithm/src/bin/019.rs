use proconio::input;

fn fact(n: u64) -> u64 {
    let mut ret: u64 = 1;
    for i in 1..=n { ret *= i; }
    return ret;
}

fn perm(n: u64, r: u64) -> u64 {
    let mut ret: u64 = 1;
    for i in 0..r { ret *= n - i; }
    return ret;
}

fn comb(n: u64, r: u64) -> u64 {
    return perm(n, r) / fact(r);
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut cnt: Vec<u64> = vec![0; 4];
    for i in 0..n {
        cnt[a[i]] += 1;
    }

    println!("{}", comb(cnt[1], 2) + comb(cnt[2], 2) + comb(cnt[3], 2));
}
