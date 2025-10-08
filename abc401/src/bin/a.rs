use proconio::input;

fn main() {
    input! {
        s: u32,
    }

    println!("{}", if s / 100 == 2 { "Success" } else { "Failure" });
}
