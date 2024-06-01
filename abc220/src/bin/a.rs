use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
    }

    let mut ans: i32 = -1;
    for i in 0..=1000 {
        if a <= c * i && c * i <= b {
            ans = c * i;
            break;
        } else if b < c * i {
            break;
        }
    }

    println!("{}", ans);
}
