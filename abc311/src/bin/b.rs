use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        d: usize,
        s: [Chars; n],
    }

    let mut ans = 0;
    let mut current_streak = 0;
    for i in 0..d {
        if (0..n).all(|j| s[j][i] == 'o') {
            current_streak += 1;
            ans = ans.max(current_streak);
        } else {
            current_streak = 0;
        }
    }

    println!("{}", ans);
}
