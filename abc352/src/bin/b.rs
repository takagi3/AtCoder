use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut pos = 0;
    let ans: Vec<usize> = s
        .iter()
        .filter_map(|&c| {
            t[pos..].iter().position(|&x| x == c).map(|i| {
                pos += i + 1;
                pos
            })
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
