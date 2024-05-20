use proconio::input;

fn main() {
    input! {
        n: u32,
    }

    let ans: u32;
    if n <= 125 {
        ans = 4
    } else if n >= 212 {
        ans = 8
    } else {
        ans = 6
    }

    println!("{}", ans);
}
