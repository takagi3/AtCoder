use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = 1;
    for i in 0..s.len() {
        for j in i + 1..=s.len() {
            let slice: Vec<char> = s[i..j].to_vec();
            let reversed_slice: Vec<char> = slice.iter().rev().cloned().collect();
            if slice == reversed_slice {
                ans = ans.max(slice.len());
            }
        }
    }

    println!("{}", ans);
}
