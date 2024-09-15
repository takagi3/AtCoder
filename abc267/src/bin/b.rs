use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut pin: Vec<bool> = vec![true; 7];
    if s[6] == '0' {
        pin[0] = false
    }
    if s[3] == '0' {
        pin[1] = false
    }
    if s[1] == '0' && s[7] == '0' {
        pin[2] = false
    }
    if s[0] == '0' && s[4] == '0' {
        pin[3] = false
    }
    if s[2] == '0' && s[8] == '0' {
        pin[4] = false
    }
    if s[5] == '0' {
        pin[5] = false
    }
    if s[9] == '0' {
        pin[6] = false
    }
    let mut cnt: u32 = 0;
    let mut split: bool = false;
    for i in 0..7 {
        if !split && pin[i] {
            split = true;
            cnt += 1;
        } else if split && !pin[i] {
            split = false;
            cnt += 1;
        }
    }
    let ans: &str = if cnt >= 3 && s[0] == '0' { "Yes" } else { "No" };

    println!("{}", ans);
}
