use proconio::input;

fn mod_pow(a: usize, b: usize, m: usize) -> usize {
    let mut p: usize = a;
    let mut ret: usize = 1;
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

fn division(a: usize, b: usize, m: usize) -> usize {
    return (a * mod_pow(b, m - 2, m) % m) % m;
}

fn ncr(n: usize, r: usize, f: Vec<usize>, m: usize) -> usize {
    return division(f[n], f[r] * f[n - r] % m, m);
}

fn main() {
    input! {
        x: usize,
        y: usize,
    }

    const MOD: usize = 1000000007;
    let mut fact: Vec<usize> = vec![1; 20000001];
    for i in 1..=2000000 {
        fact[i] = 1 * i * fact[i - 1] % MOD;
    }
    let mut ans: usize = 0;
    if (x + y) % 3 == 0 && 2 * x >= y && 2 * y >= x {
        ans = ncr((x + y) / 3, (2 * x - y) / 3, fact, MOD)
    }

    println!("{}", ans);
}
