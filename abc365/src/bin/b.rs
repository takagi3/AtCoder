use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }

    let &max = a.iter().max().unwrap();

    println!(
        "{}",
        a.iter()
            .enumerate()
            .filter(|&(_, &v)| v != max)
            .max_by_key(|&(_, &v)| v)
            .map(|(i, _)| i + 1)
            .unwrap_or(0)
    );
}
