use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut ans: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'o' {
                if ans == 0 {
                    x = j as i32;
                    y = i as i32;
                    ans = 1;
                } else {
                    ans = (x - j as i32).abs() + (y - i as i32).abs()
                }
            }
        }
    }

    println!("{}", ans);
}
