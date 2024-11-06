use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        q: usize,
        events: [(usize, Usize1); q],
    }

    let mut cards = vec![0; n];
    let ans: Vec<_> = events
        .into_iter()
        .filter_map(|(c, x)| match c {
            1 => {
                cards[x] += 1;
                None
            }
            2 => {
                cards[x] += 2;
                None
            }
            3 => Some(if cards[x] >= 2 { "Yes" } else { "No" }),
            _ => None,
        })
        .collect();

    println!("{}", ans.join("\n"));
}
