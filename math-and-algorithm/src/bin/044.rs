use std::collections::VecDeque;
use proconio::input;

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

    let mut ans: Vec<i32> = vec![-1; n + 1];
    ans[1] = 0;
    let mut q: VecDeque<usize> = VecDeque::new();
    q.push_back(1);
    let mut current: usize;
    while !q.is_empty() {
        current = q.pop_front().unwrap_or(0);
        for &next in &graph[current] {
            if ans[next] == -1 {
                ans[next] = ans[current] + 1;
                q.push_back(next);
            }
        }
    }

    for i in 1..=n {
        println!("{}", ans[i])
    }
}
