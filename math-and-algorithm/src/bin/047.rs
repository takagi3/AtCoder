use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut g: Vec<Vec<usize>> = vec![vec![]; n + 1];
    for i in 0..m {
        g[ab[i].0].push(ab[i].1);
        g[ab[i].1].push(ab[i].0);
    }

    let mut c: Vec<usize> = vec![0; n + 1];
    let mut q: VecDeque<usize> = VecDeque::new();
    q.push_back(1);
    c[0] = 3;
    c[1] = 1;

    let mut ans: &str = "Yes";
    'queue: while !q.is_empty() {
        let p: usize = q.pop_front().unwrap();
        for next in &g[p] {
            if c[*next] == 0 {
                if c[p] == 1 {
                    c[*next] = 2;
                } else if c[p] == 2 {
                    c[*next] = 1;
                }
                q.push_back(*next);
            } else if c[p] == c[*next] {
                ans = "No";
                break 'queue;
            }
        }

        let other: usize = c.iter().position(|&x| x == 0).unwrap_or(0);
        if q.is_empty() && other != 0 {
            q.push_back(other);
            c[other] = 1;
        }
    }

    println!("{}", ans);
}
