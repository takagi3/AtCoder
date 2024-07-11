use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    }

    let mut ans: &str = "No";
    if b - a == 1 || b - a == 9 {
        ans = "Yes"
    }

    println!("{}", ans);
}
