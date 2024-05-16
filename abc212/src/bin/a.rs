use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    }

    let mut ans: &str;
    if a == 0 {
        ans = "Silver";
    } else if b == 0 {
        ans = "Gold";
    } else {
        ans = "Alloy"
    }

    println!("{}", ans);
}
