use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let ans: String = s.iter()
        .map(|&c| if c == '0' { '1' } else { '0' })
        .collect();

    println!("{}", ans);
}
