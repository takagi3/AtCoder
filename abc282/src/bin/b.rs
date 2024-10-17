use std::collections::HashSet;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }

    let mut pair: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..m {
        for j in 0..n - 1 {
            for k in j + 1..n {
                if s[j][i] == 'x' && s[k][i] == 'x' {
                    pair.insert((j, k));
                }
            }
        }
    }
    let ans: usize = n * (n - 1) / 2 - pair.len();

    println!("{}", ans);
}
