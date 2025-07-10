use proconio::input;
use proconio::marker::Chars;

const N: usize = 8;

fn main() {
    input! {
        grid: [Chars; N],
    }

    let free_rows = grid.iter().filter(|row| !row.contains(&'#')).count();

    let free_cols = (0..N)
        .filter(|&i| grid.iter().all(|row| row[i] != '#'))
        .count();

    println!("{}", free_rows * free_cols);
}
