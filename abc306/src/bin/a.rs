use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let ans: String = s.iter().flat_map(|&c| std::iter::repeat(c).take(2)).collect();

    println!("{}", ans);
}
