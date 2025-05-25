use proconio::input;

fn main() {
    input! {
        n: u32,
        t: u32,
        a: u32,
    }

    println!("{}", if n / 2 < t || n / 2 < a { "Yes" } else { "No" });
}
