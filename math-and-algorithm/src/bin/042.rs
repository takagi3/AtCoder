use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut div: Vec<usize> = vec![0; n + 1];
    for i in 1..=n {
        for j in (i..=n).step_by(i) {
            div[j] += 1
        }
    }

    let mut ans: usize = 0;
    for i in 1..=n {
        ans += div[i] * i
    }

    println!("{}", ans);
}
