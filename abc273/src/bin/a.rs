use proconio::input;

fn f(x: u32) -> u32 {
    if x == 0 {
        1
    } else {
        x * f(x - 1)
    }
}

fn main() {
    input! {
        n: u32,
    }

    println!("{}", f(n));
}
