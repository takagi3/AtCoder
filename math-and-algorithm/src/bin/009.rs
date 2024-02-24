use proconio::input;

fn main() {
    input! {
        n: usize,
        s: u64,
        a: [u64; n],
    }

    for i in 0u64..(1 << n) {
        let mut sum: u64 = 0;
        for j in 0..n {
            if i & (1 << j) != 0 { sum += a[j] }
        }
        if sum == s {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
