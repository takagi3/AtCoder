use proconio::input;

fn main() {
    input! {
        n: u32,
        k: u32,
        a: u32,
    }

    let ans: u32 = if (a - 1 + k) % n != 0 { (a - 1 + k) % n } else { n };

    println!("{}", ans);
}
