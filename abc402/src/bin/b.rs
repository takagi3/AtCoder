use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        q: usize
    }

    let mut que = VecDeque::with_capacity(q);
    let mut ans = Vec::new();
    for _ in 0..q {
        input! { t: u8 }
        if t == 1 {
            input! { x: i64 }
            que.push_back(x);
        } else {
            ans.push(que.pop_front().unwrap().to_string());
        }
    }

    println!("{}", ans.join("\n"));
}
