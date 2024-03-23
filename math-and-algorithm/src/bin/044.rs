use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n + 1];
    for i in 0..m {
        graph[ab[i].0].push(ab[i].1);
        graph[ab[i].1].push(ab[i].0);
    }

    let mut deque: VecDeque<usize> = VecDeque::new();
    deque.push_back(1);
    let mut dist: Vec<i32> = vec![-1; n + 1];
    dist[1] = 0;
    let mut p: usize;
    while deque.len() > 0 {
        p = deque.pop_front().unwrap_or(0);
        for next in graph[p].iter() {
            if dist[*next] == -1 {
                dist[*next] = dist[p] + 1;
                deque.push_back(*next);
            }
        }
    }

    for i in 1..=n {
        println!("{}", dist[i]);
    }
}
