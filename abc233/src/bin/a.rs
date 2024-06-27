use proconio::input;

fn main() {
    input! {
        x: u32,
        y: u32,
    }

    let mut ans: u32 = 0;
    if x < y {
        ans = (y - x) / 10;
        if (y - x) % 10 > 0 {
            ans += 1
        }
    }

    println!("{}", ans);
}
