use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        r: usize,
        c: usize,
        b: [Chars; r],
    }

    let mut ans = vec![vec!['#'; c]; r];
    for i in 0..r {
        for j in 0..c {
            match b[i][j] {
                '.' => ans[i][j] = '.',
                '#' => continue,
                bomb if bomb.is_digit(10) => {
                    let range = bomb.to_digit(10).unwrap() as i32;
                    for k in 0..r {
                        for l in 0..c {
                            if (i as i32 - k as i32).abs() + (j as i32 - l as i32).abs() <= range {
                                ans[k][l] = '.';
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    for row in ans {
        println!("{}", row.iter().collect::<String>());
    }
}
