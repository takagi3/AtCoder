use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
        a: [usize; n],
    }

    let ans: Vec<usize> = a
        .iter()
        .map(|&x| match x {
            x if x <= l => l,
            x if x >= r => r,
            _ => x,
        })
        .collect();

    println!(
        "{}",
        ans.iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    );
}
