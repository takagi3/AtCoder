use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let m = s.iter().map(Vec::len).max().unwrap();
    let rev = s.iter().rev().collect::<Vec<_>>();
    for j in 0..m {
        let row = rev
            .iter()
            .map(|v| v.get(j).cloned().unwrap_or('*'))
            .collect::<Vec<_>>();
        let last = row.iter().rposition(|&c| c != '*').unwrap();
        let line = row[..=last].iter().collect::<String>();
        println!("{}", line);
    }
}
