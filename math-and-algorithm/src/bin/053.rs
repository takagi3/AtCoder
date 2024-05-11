use proconio::input;

fn mod_pow(a: isize, b: isize, m: isize) -> isize {
    let mut p: isize = a;
    let mut ret: isize = 1;
    for i in 0..64 {
        if b & (1 << i) != 0 {
            ret *= p;
            ret %= m;
        }
        p *= p;
        p %= m;
    }
    return ret;
}

fn division(a: isize, b: isize, m: isize) -> isize {
    return a * mod_pow(b, m - 2, m) % m;
}

fn main() {
    input! {
        n: isize,
    }

    const MOD: isize = 1000000007;
    let ans: isize = division(1 - mod_pow(4, n + 1, MOD), -3, MOD);

    println!("{}", ans);
}
