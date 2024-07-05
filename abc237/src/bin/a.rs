use proconio::input;

fn main() {
    input! {
        n: i64,
    }

    let mut ans: &str = "No";
    if -2i64.pow(31) <= n && n < 2i64.pow(31) as i64 {
        ans = "Yes";
    }

    println!("{}", ans);
}
