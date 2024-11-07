use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let ans: String = s.chunks(2)
        .flat_map(|pair| pair.iter().rev())
        .collect();

    println!("{}", ans);
}
