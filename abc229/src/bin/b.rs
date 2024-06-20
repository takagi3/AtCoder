use proconio::input;

fn main() {
    input! {
        mut a: u64,
        mut b: u64,
    }

    let mut ans: &str = "Easy";
    for _ in 0..18 {
        if a % 10 + b % 10 > 9 {
            ans = "Hard";
            break;
        }
        a /= 10;
        b /= 10;
        if a == 0 || b == 0 {
            break;
        }
    }

    println!("{}", ans);
}
