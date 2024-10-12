use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [i32; n],
    }

    let mut ans: Vec<i32> = vec![0; n];
    ans[0] = s[0];
    for i in 1..n {
        ans[i] = s[i] - s[i - 1]
    }

    println!("{}", ans.iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" ")
    );
}
