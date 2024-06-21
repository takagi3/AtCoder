use proconio::input;

fn main() {
    input! {
        n: u32,
    }

    let ans: u32;
    if n < 42 {
        ans = n
    } else {
        ans = n + 1
    }

    println!("AGC{0: >03}", ans);
}
