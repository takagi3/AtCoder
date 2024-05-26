use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }

    let ans: &str;
    if s < t {
        ans = "Yes"
    } else {
        ans = "No"
    }

    println!("{}", ans);
}
