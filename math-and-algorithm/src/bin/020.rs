use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    let mut ans: u64 = 0;
    for i in 0..n - 4 {
        for j in i + 1..n - 3 {
            for k in j + 1..n - 2 {
                for l in k + 1..n - 1 {
                    for m in l + 1..n {
                        if a[i] + a[j] + a[k] + a[l] + a[m] == 1000 {
                            ans += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
