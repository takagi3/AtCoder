use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [u32; n],
    }

    let mut ans: Vec<u32> = vec![0; n];
    for i in 0..n {
        ans[i] = if i + k < n { a[i + k] } else { 0 }
    }

    println!("{}", ans.iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" ")
    );
}
