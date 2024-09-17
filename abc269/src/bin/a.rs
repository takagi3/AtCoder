use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        d: i32,
    }

    let ans1: i32 = (a + b) * (c - d);
    let ans2: &str = "Takahashi";

    println!("{}\n{}", ans1, ans2);
}
