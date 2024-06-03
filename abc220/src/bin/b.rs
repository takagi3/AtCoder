use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        k: i64,
        mut a: Chars,
        mut b: Chars,
    }

    a.reverse();
    let mut a_num: i64 = 0;
    for i in 0..a.len() {
        a_num += (a[i] as i64 - 48) * k.pow(i as u32)
    }
    b.reverse();
    let mut b_num: i64 = 0;
    for i in 0..b.len() {
        b_num += (b[i] as i64 - 48) * k.pow(i as u32)
    }
    let ans: i64 = a_num * b_num;

    println!("{}", ans);
}
