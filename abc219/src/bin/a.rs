use proconio::input;

fn main() {
    input! {
        x: u32,
    }

    let ans: String;
    if x < 40 {
        ans = (40 - x).to_string()
    } else if x < 70 {
        ans = (70 - x).to_string()
    } else if x < 90 {
        ans = (90 - x).to_string()
    } else {
        ans = "expert".to_string()
    }

    println!("{}", ans);
}
