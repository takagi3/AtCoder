use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[usize; n]; m],
    }

    let mut adjacency = vec![HashSet::new(); n + 1];
    for row in &a {
        row.windows(2).for_each(|w| {
            adjacency[w[0]].insert(w[1]);
            adjacency[w[1]].insert(w[0]);
        });
    }
    let total_missing_connections: usize = adjacency
        .iter()
        .skip(1)
        .map(|neighbors| n - 1 - neighbors.len())
        .sum();

    println!("{}", total_missing_connections / 2);
}
