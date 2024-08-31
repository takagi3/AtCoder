use proconio::input;

fn main() {
    input! {
        l1: u32,
        r1: u32,
        l2: u32,
        r2: u32,
    }

    let mut ans: u32 = 0;
    for i in l1..=r1 {
        if l2 <= i && i <= r2 {
            ans += 1
        }
    }
    if ans > 0 {
        ans -= 1
    }

    println!("{}", ans);
}
