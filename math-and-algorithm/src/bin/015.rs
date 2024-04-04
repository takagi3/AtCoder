use proconio::input;
use num::integer::gcd;

fn main() {
    input! {
        a: u64,
        b: u64,
    }

    println!("{}", gcd(a, b));
}
