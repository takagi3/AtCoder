use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let ans = if s.first().map(|&c| c.is_uppercase()).unwrap_or(false)
        && s.iter().skip(1).all(|&c| c.is_lowercase())
    {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}
