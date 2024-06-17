use proconio::input;

fn main() {
    input! {
        s: u32,
        mut t: u32,
        x: u32,
    }

    let ans: &str;
    if t < s {
        t += 24
    }
    if s <= x && x < t || s <= x + 24 && x + 24 < t {
        ans = "Yes"
    } else {
        ans = "No"
    }

    println!("{}", ans);
}
