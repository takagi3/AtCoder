use proconio::input;

fn main() {
    input! {
        n: u64,
        s: u64,
    }

    let mut ans: u64 = 0;

    for i in 1..=n {
        for j in 1..=n {
            if i + j <= s { ans += 1; }
        }
    }

    println!("{}", ans);
}
