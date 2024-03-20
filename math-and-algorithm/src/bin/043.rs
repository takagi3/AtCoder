use proconio::input;

fn dfs(next: usize, graph: Vec<Vec<usize>>, mut visited: Vec<bool>) -> Vec<bool> {
    graph[next].iter().for_each(|x| {
        if visited[*x] == false {
            visited[*x] = true;
            visited = dfs(*x, graph.clone(), visited.clone())
        }
    });

    return visited;
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

    let mut v: Vec<bool> = vec![false; n + 1];
    v[1] = true;
    v = dfs(1, g.clone(), v.clone());

    let mut ans: &str = "The graph is connected.";
    for i in 1..=n {
        if !v[i] {
            ans = "The graph is not connected.";
            break;
        }
    }

    println!("{}", ans);
}
