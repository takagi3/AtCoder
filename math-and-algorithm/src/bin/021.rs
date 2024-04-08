use proconio::input;

fn factorial(n: u64) -> u64 {
    let mut ans: u64 = 1;
    for i in 1..=n {
        ans *= i;
    }
    return ans;
}

fn perm(n: u64, r: u64) -> u64 {
    let mut ans: u64 = 1;
    for i in 0..r {
        ans *= n - i;
    }
    return ans;
}

fn comb(n: u64, r: u64) -> u64 {
    return perm(n, r) / factorial(r);
}

fn main() {
    input! {
        n: u64,
        r: u64,
    }

    let ans: u64 = comb(n, r);

    println!("{}", ans);
}
