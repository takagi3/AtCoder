use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let ans = if s.first() == Some(&'<')
        && s.last() == Some(&'>')
        && s[1..s.len() - 1].iter().all(|&c| c == '=')
    {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}
