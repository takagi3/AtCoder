use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        queries: [(Usize1, char); m],
    }

    let mut available = vec![true; n];
    for (idx, c) in queries {
        if available[idx] && c == 'M' {
            available[idx] = false;
            println!("Yes");
        } else {
            println!("No");
        }
    }
}