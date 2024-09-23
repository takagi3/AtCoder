use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        q: usize,
        sequences: [[usize]; n],
        query: [(Usize1, Usize1); q],
    }

    for (i, j) in query {
        println!("{}", sequences[i][j]);
    }
}
