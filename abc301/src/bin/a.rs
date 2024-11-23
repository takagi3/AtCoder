use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        a: Chars
    }

    let mut ans: char = 'A';
    let mut cnt_a: u32 = 0;
    let mut cnt_t: u32 = 0;
    for &p in &a {
        match p {
            'A' => cnt_a += 1,
            'T' => cnt_t += 1,
            _ => {}
        }
        if cnt_a > cnt_t {
            ans = 'A'
        } else if cnt_a < cnt_t {
            ans = 'T'
        }
    }

    println!("{}", ans);
}
