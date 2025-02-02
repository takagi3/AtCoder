use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut ans: u32 = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                ans += 1
            }
        }
    }

    println!("{}", ans);
}
