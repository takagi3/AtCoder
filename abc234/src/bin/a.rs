use proconio::input;

fn f(x: u32) -> u32 {
    x * x + 2 * x + 3
}

fn main() {
    input! {
        t: u32,
    }

    let ans: u32 = f(f(f(t) + t) + f(f(t)));

    println!("{}", ans);
}
