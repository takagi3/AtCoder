use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let cnt_upper = s.iter().filter(|c| c.is_uppercase()).count();
    let ans: String = if cnt_upper > s.len() - cnt_upper {
        s.iter().map(|&c| c.to_ascii_uppercase()).collect()
    } else {
        s.iter().map(|&c| c.to_ascii_lowercase()).collect()
    };

    println!("{}", ans);
}
