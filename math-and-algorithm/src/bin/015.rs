use proconio::input;

fn gcd(a: u64, b: u64) -> u64 {
    return if a % b == 0 { b } else { gcd(b, a % b) }
}

fn main() {
    input! {
        a: u64,
        b: u64,
    }

    println!("{}", gcd(a, b));
}
