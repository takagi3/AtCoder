use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: [Chars; 8],
    }

    let mut ans = String::new();
    for (i, row) in s.iter().enumerate() {
        if let Some(j) = row.iter().position(|&c| c == '*') {
            ans.push((b'a' + j as u8) as char);
            ans.push((b'8' - i as u8) as char);
        }
    }

    println!("{}", ans);
}
