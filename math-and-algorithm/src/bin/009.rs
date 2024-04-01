use proconio::input;

fn main() {
    input! {
        n: usize,
        s: u32,
        a: [u32; n],
    }

    let mut ans: &str = "No";
    for i in 0u64..(1 << n) {
        let mut tmp_sum: u32 = 0;
        for j in 0..n {
            if i & (1 << j) != 0 {
                tmp_sum += a[j]
            }
        }
        if tmp_sum == s {
            ans = "Yes";
            break;
        }
    }

    println!("{}", ans);
}
