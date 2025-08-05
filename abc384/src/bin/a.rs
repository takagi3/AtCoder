use proconio::input;

fn main() {
    input! {
        _n: usize,
        c1: char,
        c2: char,
        s: String,
    }

    let ans: String = s.chars().map(|c| if c == c1 { c } else { c2 }).collect();

    println!("{}", ans);
}
