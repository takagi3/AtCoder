use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: [Chars; 10],
    }

    let mut a: usize = 0;
    let mut b: usize = 0;
    let mut c: usize = 0;
    let mut d: usize = 0;
    'out1: for i in 0..10 {
        for j in 0..10 {
            if s[i][j] == '#' {
                a = i + 1;
                c = j + 1;
                break 'out1;
            }
        }
    }
    'out2: for i in (0..10).rev() {
        for j in (0..10).rev() {
            if s[i][j] == '#' {
                b = i + 1;
                d = j + 1;
                break 'out2;
            }
        }
    }

    println!("{} {}\n{} {}", a, b, c, d);
}
