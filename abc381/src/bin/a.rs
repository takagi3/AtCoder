use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let ok = if n % 2 == 1 {
        let half = n / 2;
        s[..half].iter().all(|&c| c == '1')
            && s[half] == '/'
            && s[half + 1..].iter().all(|&c| c == '2')
    } else {
        false
    };

    println!("{}", if ok { "Yes" } else { "No" });
}
