use proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize,
    }

    let base: String = String::from("atcoder");
    let ans: &str = &base[l - 1..r];

    println!("{}", ans);
}
