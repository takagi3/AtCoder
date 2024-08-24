use proconio::input;

fn main() {
    input! {
        n: u32,
        x: u32,
    }

    let ans: char = ((x - 1) / n + 65) as u8 as char;

    println!("{}", ans);
}
