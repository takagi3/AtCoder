use proconio::input;

fn gcd(a: u64, b: u64) -> u64 {
    return if a % b == 0 { b } else { gcd(b, a % b) }
}

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    let mut ans: u64 = a[0];
    for i in 1..n {
        ans = gcd(a[i], ans);
    }

    println!("{}", ans);
}
