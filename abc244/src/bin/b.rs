use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        t: Chars,
    }

    let dx: Vec<i32> = vec![1, 0, -1, 0];
    let dy: Vec<i32> = vec![0, -1, 0, 1];
    let mut dt: usize = 0;
    let mut ans: (i32, i32) = (0, 0);
    for i in 0..n {
        if t[i] == 'S' {
            ans.0 += dx[dt];
            ans.1 += dy[dt];
        } else if t[i] == 'R' {
            dt = (dt + 1) % 4
        }
    }

    println!("{} {}", ans.0, ans.1);
}
