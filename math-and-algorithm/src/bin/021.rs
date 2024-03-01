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
        n: u64,
        r: u64,
    }

    println!("{}", comb(n, r));
}
