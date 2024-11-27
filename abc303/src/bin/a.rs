use proconio::input;
use proconio::marker::Chars;

fn is_similar(c1: char, c2: char) -> bool {
    c1 == c2 || (c1 == '1' && c2 == 'l') || (c1 == 'l' && c2 == '1') || (c1 == '0' && c2 == 'o') || (c1 == 'o' && c2 == '0')
}

fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    }

    let ans = if (0..n).all(|i| is_similar(s[i], t[i])) {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}
