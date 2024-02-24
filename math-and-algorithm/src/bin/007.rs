use proconio::input;

fn main() {
    input! {
        n: u64,
        x: u64,
        y: u64,
    }

    let mut ans: u64 = 0;

    for i in 1..=n {
        if i % x == 0 || i % y == 0 { ans += 1; }
    }

    println!("{}", ans);
}
