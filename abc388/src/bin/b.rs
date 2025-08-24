use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        tl: [(usize, usize); n],
    }

    let base: Vec<usize> = tl.iter().map(|&(t, l)| t * l).collect();
    let ts: Vec<usize> = tl.iter().map(|&(t, _)| t).collect();

    for k in 1..=d {
        println!(
            "{}",
            base.iter().zip(&ts).map(|(b, &t)| b + t * k).max().unwrap()
        );
    }
}
