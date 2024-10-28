use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n],
    }

    for i in 0..n {
        println!("{}", ab[i].0 + ab[i].1);
    }
}
