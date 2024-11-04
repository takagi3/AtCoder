use proconio::input;

fn main() {
    input! {
        n: usize,
        mut x: [i32; 5 * n],
    }

    x.sort();
    let total: f64 = x[n..4 * n].iter().map(|&v| v as f64).sum();

    println!("{}", total / (3 * n) as f64);
}
