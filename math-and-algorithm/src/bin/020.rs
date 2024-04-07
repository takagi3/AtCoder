use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }

    let mut ans: u32 = 0;
    for i in 0..n - 4 {
        for j in i + 1..n - 3 {
            for k in j + 1..n - 2 {
                for l in k + 1..n - 1 {
                    for m in l + 1..n {
                        let sum: u32 = a[i] + a[j] + a[k] + a[l] + a[m];
                        if sum == 1000 {
                            ans += 1
                        }
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
