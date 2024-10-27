use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [usize; n],
        t: [usize; m],
    }

    let mut ans: u32 = 0;
    for i in 0..n {
        if t.contains(&(s[i] % 1000)) {
            ans += 1
        }
    }

    println!("{}", ans);
}
