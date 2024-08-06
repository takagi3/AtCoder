use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
    }

    let mut ans: &str = "No";
    if a <= b && b <= c || a >= b && b >= c {
        ans = "Yes"
    }

    println!("{}", ans);
}
