use proconio::input;
use std::cmp::Ordering;

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
    }

    let (t, a): (usize, usize) = xy.iter().fold((0, 0), |(t, a), &(x, y)| (t + x, a + y));

    match t.cmp(&a) {
        Ordering::Greater => println!("Takahashi"),
        Ordering::Less => println!("Aoki"),
        Ordering::Equal => println!("Draw"),
    }
}
