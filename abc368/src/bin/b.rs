use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    let mut heap = a.into_iter().filter(|&x| x > 0).collect::<BinaryHeap<_>>();
    let mut ans = 0;
    while heap.len() > 1 {
        let mut x = heap.pop().unwrap();
        let mut y = heap.pop().unwrap();
        x -= 1;
        y -= 1;
        if x > 0 {
            heap.push(x);
        }
        if y > 0 {
            heap.push(y);
        }
        ans += 1;
    }

    println!("{}", ans);
}
