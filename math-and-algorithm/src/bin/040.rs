use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n-1],
        m: usize,
        b: [usize; m],
    }

    let mut fd: Vec<i64> = vec![0; n];
    for i in 1..n {
        fd[i] = fd[i - 1] + a[i - 1]
    }
    
    let mut ans: i64 = 0;
    for i in 1..m {
        ans += (fd[b[i] - 1] - fd[b[i - 1] - 1]).abs()
    }

    println!("{}", ans);
}
