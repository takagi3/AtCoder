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

    let mut c: Vec<usize> = vec![0; n + 1];
    let mut q: VecDeque<usize> = VecDeque::new();
    q.push_back(1);
    c[0] = 3;
    c[1] = 1;

    let mut ans: &str = "Yes";
    'queue: while !q.is_empty() {
        let p: usize = q.pop_front().unwrap();
        for &next in &graph[p] {
            if c[next] == 0 {
                c[next] = if c[p] == 1 { 2 } else { 1 };
                q.push_back(next);
            }
            if c[p] == c[next] {
                ans = "No";
                break 'queue;
            }
        }


        if q.is_empty() {
            let other: usize = c.iter().position(|&x| x == 0).unwrap_or(0);
            if other != 0 {
                q.push_back(other);
                c[other] = 1;
            }
        }
    }

    println!("{}", ans);
}
