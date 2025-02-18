use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut expected: char = 'A';
    for &c in &s {
        if c < expected {
            println!("No");
            return;
        }
        if c == 'A' {
            expected = 'A';
        } else if c == 'B' {
            expected = 'B';
        } else if c == 'C' {
            expected = 'C';
        }
    }
    println!("Yes");
}
