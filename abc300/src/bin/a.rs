use proconio::input;

fn main() {
    input! {
        n: usize,
        a: u32,
        b: u32,
        c: [u32; n],
    }

    if let Some(i) = c.iter().position(|&x| x == a + b) {
        println!("{}", i + 1);
    }
}
