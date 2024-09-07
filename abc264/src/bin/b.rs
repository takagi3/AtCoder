use proconio::input;

fn main() {
    input! {
        r: usize,
        c: usize,
    }

    let size: usize = 15;
    let mut pattern: Vec<Vec<&str>> = vec![vec![""; size]; size];
    for i in 0..size {
        for j in 0..size {
            let layer: usize = i.min(j).min(size - 1 - i).min(size - 1 - j);
            if layer % 2 == 0 {
                pattern[i][j] = "black"
            } else {
                pattern[i][j] = "white"
            }
        }
    }

    println!("{}", pattern[r - 1][c - 1]);
}
