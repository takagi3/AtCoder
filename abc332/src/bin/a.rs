use proconio::input;

fn main() {
    input! {
        n: usize,
        s: u32,
        k: u32,
        pq: [(u32, u32); n],
    }

    let ans: u32 = pq.into_iter().map(|(p, q)| p * q).sum();

    println!("{}", ans + if ans < s { k } else { 0 });
}
