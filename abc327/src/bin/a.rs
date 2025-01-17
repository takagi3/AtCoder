use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let ans = if s.windows(2).any(|c| matches!(c, ['a', 'b'] | ['b', 'a'])) {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}
