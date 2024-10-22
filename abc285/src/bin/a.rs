use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    }

    let ans: &str = if a * 2 == b || a * 2 + 1 == b {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}
