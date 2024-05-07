use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    const MOD: u64 = 1000000007;
    let mut fibonacci: Vec<u64> = vec![1; n + 1];
    for i in 3..=n {
        fibonacci[i] = (fibonacci[i - 1] + fibonacci[i - 2]) % MOD;
    }

    println!("{}", fibonacci[n]);
}
