use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        abc: Chars,
    }

    let mut ans: i32 = 0;
    ans += (abc[0] as i32 - 48) * 100 + (abc[1] as i32 - 48) * 10 + (abc[2] as i32 - 48);
    ans += (abc[1] as i32 - 48) * 100 + (abc[2] as i32 - 48) * 10 + (abc[0] as i32 - 48);
    ans += (abc[2] as i32 - 48) * 100 + (abc[0] as i32 - 48) * 10 + (abc[1] as i32 - 48);

    println!("{}", ans);
}
