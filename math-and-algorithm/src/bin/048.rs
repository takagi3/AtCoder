use std::cmp::Reverse;
use std::collections::BinaryHeap;
use proconio::input;

fn main() {
    input! {
        k: usize,
    }

    let mut graph: Vec<Vec<(usize, usize)>> = vec![vec![]; k];
    for i in 0..k {
        for j in 0..10 {
            if i == 0 && j == 0 { continue; }
            graph[i].push(((i * 10 + j) % k, j));
        }
    }

    let mut dist: Vec<usize> = vec![10_usize.pow(10); k];
    let mut used: Vec<bool> = vec![false; k];
    let mut q: BinaryHeap<(Reverse<usize>, usize)> = BinaryHeap::new();
    q.push((Reverse(0), 0));

    while !q.is_empty() {
        let pos: usize = q.pop().unwrap().1;
        if used[pos] == true { continue; }
        used[pos] = true;
        for x in &graph[pos] {
            let to = x.0;
            let mut cost = dist[pos] + x.1;
            if pos == 0 {
                cost = x.1;
            }
            if dist[to] > cost {
                dist[to] = cost;
                q.push((Reverse(dist[to]), to));
            }
        }
    }

    println!("{}", dist[0]);
}
