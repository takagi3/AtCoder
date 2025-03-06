use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
        q: usize,
        ab: [(usize, usize); q],
    }

    let index = p
        .into_iter()
        .enumerate()
        .map(|(i, x)| (x, i))
        .collect::<HashMap<_, _>>();
    ab.into_iter()
        .for_each(|(a, b)| println!("{}", if index[&a] < index[&b] { a } else { b }));
}
