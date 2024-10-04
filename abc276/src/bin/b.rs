use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut ans: Vec<Vec<usize>> = vec![vec![]; n];
    for i in 0..m {
        ans[ab[i].0 - 1].push(ab[i].1);
        ans[ab[i].1 - 1].push(ab[i].0);
    }

    for i in 0..n {
        print!("{} ", ans[i].len());
        ans[i].sort();
        println!("{}", ans[i].iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ")
        );
    }
}
