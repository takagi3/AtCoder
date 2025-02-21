use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut cnt = vec![0; 26];
    for c in s {
        cnt[(c as u8 - b'a') as usize] += 1;
    }

    let (i, _) = cnt
        .iter()
        .enumerate()
        .rev()
        .max_by_key(|&(_, &v)| v)
        .unwrap();
    println!("{}", (b'a' + i as u8) as char);
}
