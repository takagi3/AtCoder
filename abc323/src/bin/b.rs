use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let mut cnt: Vec<(usize, usize)> = s
        .iter()
        .enumerate()
        .map(|(i, row)| (row.iter().filter(|&&c| c == 'o').count(), i + 1))
        .collect();

    cnt.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));

    let ans: Vec<String> = cnt.iter().map(|&(_, i)| i.to_string()).collect();
    println!("{}", ans.join(" "));
}
