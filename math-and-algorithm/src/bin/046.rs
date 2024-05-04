use std::collections::VecDeque;
use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        r: usize,
        c: usize,
        start: (usize, usize),
        goal: (usize, usize),
        map: [Chars; r],
    }

    let mut graph: Vec<Vec<Vec<(usize, usize)>>> = vec![vec![vec![]; c]; r];

    for i in 1..r - 1 {
        for j in 1..c - 1 {
            if map[i][j] == '.' {
                if map[i - 1][j] == '.' {
                    graph[i][j].push((i - 1, j))
                }
                if map[i + 1][j] == '.' {
                    graph[i][j].push((i + 1, j))
                }
                if map[i][j - 1] == '.' {
                    graph[i][j].push((i, j - 1))
                }
                if map[i][j + 1] == '.' {
                    graph[i][j].push((i, j + 1))
                }
            }
        }
    }

    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back((start.0 - 1, start.1 - 1));
    let mut dist: Vec<Vec<u32>> = vec![vec![u32::MAX; c]; r];
    dist[start.0 - 1][start.1 - 1] = 0;

    while !q.is_empty() {
        let p = q.pop_front().unwrap();
        for &nex in &graph[p.0][p.1] {
            if dist[nex.0][nex.1] > dist[p.0][p.1] + 1 {
                dist[nex.0][nex.1] = dist[p.0][p.1] + 1;
                q.push_back((nex.0, nex.1));
            }
        }
    }

    println!("{}", dist[goal.0 - 1][goal.1 - 1]);
}
