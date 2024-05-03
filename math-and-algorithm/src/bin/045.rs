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
    let mut ans: u32 = 0;
    for i in 1..=n {
        let mut cnt: u32 = 0;
        for &x in &graph[i] {
            if x < i {
                cnt += 1
            }
        }
        if cnt == 1 {
            ans += 1
        }
    }

    println!("{}", ans);
}
