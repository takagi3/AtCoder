use proconio::input;

fn main() {
    input! {
        t: usize,
        n: usize,
        lr: [(usize, usize); n],
    }

    let mut staff: Vec<i32> = vec![0; t + 1];
    for i in 0..n {
        staff[lr[i].0] += 1;
        staff[lr[i].1] -= 1;
    }

    let mut ans: i32 = 0;
    for i in 0..t {
        ans += staff[i];
        println!("{}", ans);
    }
}
