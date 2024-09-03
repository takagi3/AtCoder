use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m],
    }

    let mut ans: u32 = 0;
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n + 1];
    for i in 0..m {
        graph[uv[i].0].push(uv[i].1);
        graph[uv[i].1].push(uv[i].0);
    }
    for i in 1..=n {
        for &j in &graph[i] {
            if i > j { continue; }
            for &k in &graph[j] {
                if j > k { continue; }
                for &l in &graph[k] {
                    if i == l { ans += 1 }
                }
            }
        }
    }

    println!("{}", ans);
}
