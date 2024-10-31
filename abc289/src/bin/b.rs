use std::collections::VecDeque;
use proconio::input;

fn main() {
    input! {
        n: u32,
        m: usize,
        a: [u32; m],
    }

    let mut ans = Vec::with_capacity(n as usize);
    let mut que = VecDeque::new();
    let mut idx = 0;

    for i in 1..=n {
        if idx < m && i == a[idx] {
            que.push_back(i);
            idx += 1;
        } else {
            ans.push(i);
            while let Some(x) = que.pop_back() {
                ans.push(x);
            }
        }
    }

    println!("{}", ans.iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" ")
    );
}
