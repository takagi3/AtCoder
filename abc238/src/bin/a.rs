use proconio::input;

fn main() {
    input! {
        n: u32,
    }

    let mut ans: &str = "Yes";
    if 1 < n && n < 5 {
        ans = "No"
    }

    println!("{}", ans);
}
