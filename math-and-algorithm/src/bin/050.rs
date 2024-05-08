use proconio::input;

fn mod_pow(a: u64, b: u64, m: u64) -> u64 {
    let mut p: u64 = a;
    let mut ret: u64 = 1;
    for i in 0..30 {
        if b & (1 << i) != 0 {
            ret *= p;
            ret %= m;
        }
        p *= p;
        p %= m;
    }
    return ret;
}

fn main() {
    input! {
        a: u64,
        b: u64,
    }

    const MOD: u64 = 1000000007;

    println!("{}", mod_pow(a, b, MOD));
}
