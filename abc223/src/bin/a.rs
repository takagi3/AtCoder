use proconio::input;

fn main() {
    input! {
        x: u32,
    }

    let mut ans: &str = "No";
    if x != 0 && x % 100 == 0 {
        ans = "Yes"
    }

    println!("{}", ans);
}
