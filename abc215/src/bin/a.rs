use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut ans: &str = "WA";
    if s == "Hello,World!" {
        ans = "AC"
    }

    println!("{}", ans);
}
