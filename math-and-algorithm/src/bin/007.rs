use proconio::input;

fn main() {
    input! {
        n: u32,
        x: u32,
        y: u32,
    }

    let mut ans: u32 = 0;
    for i in 1..=n {
        if i % x == 0 || i % y == 0 {
            ans += 1
        }
    }

    println!("{}", ans);
}
