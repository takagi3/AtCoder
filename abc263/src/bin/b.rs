use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n - 1],
    }

    let mut pi: Vec<usize> = vec![1; n + 1];
    for i in 2..n + 1 {
        pi[i] = p[i - 2]
    }
    let mut next: usize = p[n - 2];
    let mut ans: u32 = 1;
    for _ in 0..n {
        if next == 1 {
            break;
        }
        next = pi[next];
        ans += 1;
    }

    println!("{}", ans);
}
