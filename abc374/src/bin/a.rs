use proconio::input;

fn main() {
    input! {
        s: String,
    }

    println!("{}", if s.ends_with("san") { "Yes" } else { "No" })
}
