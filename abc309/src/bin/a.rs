use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    }

    println!("{}", if (a - 1) / 3 == (b - 1) / 3 { "Yes" } else { "No" });
}
