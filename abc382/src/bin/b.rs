use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        d: usize,
        mut s: Chars,
    }

    s.iter_mut()
        .rev()
        .filter(|c| **c == '@')
        .take(d)
        .for_each(|c| *c = '.');

    println!("{}", s.into_iter().collect::<String>());
}
