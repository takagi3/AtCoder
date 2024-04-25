use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u32; n],
        lr: [(usize, usize); q],
    }

    let mut b: Vec<u32> = vec![0; n + 1];
    for i in 0..n {
        b[i + 1] = a[i] + b[i];
    }
    let mut ans: Vec<u32> = vec![0; q];
    for i in 0..q {
        ans[i] = b[lr[i].1] - b[lr[i].0 - 1];
    }

    println!("{}", ans.iter()
        .map(std::string::ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n")
    );
}
