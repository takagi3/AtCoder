use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    println!("{}", s.windows(3).filter(|w| w == &['#', '.', '#']).count());
}
