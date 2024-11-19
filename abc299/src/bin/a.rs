use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let mut ans = "out";
    let mut ok = false;
    for c in s.iter() {
        match c {
            '|' => ok ^= true,
            '*' if ok => {
                ans = "in";
                break;
            }
            _ => {}
        }
    }

    println!("{}", ans);
}
