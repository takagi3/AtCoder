use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        k: usize,
        s: Chars,
    }

    let mut cnt: usize = 0;
    let ans: String = s.iter()
        .map(|&c| {
            if cnt < k && c == 'o' {
                cnt += 1;
                'o'
            } else {
                'x'
            }
        })
        .collect();

    println!("{}", ans);
}
