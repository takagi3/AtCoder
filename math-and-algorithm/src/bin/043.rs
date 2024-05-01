use proconio::input;

fn dfs(current: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
    visited[current] = true;
    for &next in &graph[current] {
        if !visited[next] {
            dfs(next, graph, visited)
        }
    }
}

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
    let mut visited: Vec<bool> = vec![false; n + 1];
    visited[0] = true;
    dfs(1, &g, &mut visited);

    let mut ans: &str = "The graph is connected.";
    for i in 0..=n {
        if !visited[i] {
            ans = "The graph is not connected.";
            break;
        }
    }

    println!("{}", ans);
}
