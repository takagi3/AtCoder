use proconio::input;

fn main() {
    input! {
        t: usize,
        n: usize,
        lr: [(usize, usize); n],
    }

    let mut b: Vec<i32> = vec![0; t + 1];
    for i in 0..n {
        b[lr[i].0] += 1;
        b[lr[i].1] -= 1;
    }
    let mut ans: Vec<i32> = vec![0; t];
    ans[0] = b[0];
    for i in 1..t {
        ans[i] = ans[i - 1] + b[i]
    }

    println!("{}", ans.iter()
        .map(std::string::ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n")
    );
}
