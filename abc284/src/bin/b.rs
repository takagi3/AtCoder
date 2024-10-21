use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            a: [u32; n],
        }

        let mut ans: u32 = 0;
        for i in 0..n {
            if a[i] % 2 == 1 {
                ans += 1
            }
        }

        println!("{}", ans);
    }
}
