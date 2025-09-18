use proconio::input;

fn main() {
    input! {
        n: usize
    }

    for r in 0..n {
        let mut row = String::with_capacity(n);
        for c in 0..n {
            let d = r.min(c).min(n - 1 - r).min(n - 1 - c);
            row.push(if d % 2 == 0 { '#' } else { '.' });
        }
        println!("{}", row);
    }
}
