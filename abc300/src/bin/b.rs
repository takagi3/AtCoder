use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
        b: [Chars; h],
    }

    let mut ans: &str = "No";
    'nest: for i in 0..h {
        for j in 0..w {
            if (0..h).all(|k| (0..w).all(|l| a[(i + k) % h][(j + l) % w] == b[k][l])) {
                ans = "Yes";
                break 'nest;
            }
        }
    }

    println!("{}", ans);
}
