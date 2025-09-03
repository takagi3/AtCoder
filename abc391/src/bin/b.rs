use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
        t: [Chars; m],
    }

    for i in 0..=n - m {
        for j in 0..=n - m {
            let ok = (0..m).all(|k| &s[i + k][j..j + m] == &t[k][..]);
            if ok {
                println!("{} {}", i + 1, j + 1);
                return;
            }
        }
    }
}
