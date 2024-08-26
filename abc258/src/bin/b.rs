use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    }

    let mut grid: Vec<Vec<char>> = vec![vec!['0'; 3 * n]; 3 * n];
    for i in 0..3 * n {
        for j in 0..3 * n {
            grid[i][j] = a[i % n][j % n]
        }
    }
    let dx: Vec<isize> = vec![0, 1, 1, 1, 0, -1, -1, -1];
    let dy: Vec<isize> = vec![-1, -1, 0, 1, 1, 1, 0, -1];
    let mut ans: usize = 0;
    let mut tmp: String;
    let mut x: isize;
    let mut y: isize;
    for i in n..2 * n {
        for j in n..2 * n {
            for k in 0..8 {
                tmp = String::new();
                x = j as isize;
                y = i as isize;
                for _ in 0..n {
                    tmp.push(grid[y as usize][x as usize]);
                    x += dx[k];
                    y += dy[k];
                }
                if ans < tmp.parse().unwrap() {
                    ans = tmp.parse().unwrap()
                }
            }
        }
    }

    println!("{}", ans);
}
