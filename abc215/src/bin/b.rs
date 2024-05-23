use proconio::input;

fn main() {
    input! {
        n: i64,
    }

    let mut ans: u32 = 0;
    for i in 0..=64 {
        if 2_i64.pow(i) > n {
            ans = i - 1;
            break;
        }
    }

    println!("{}", ans);
}
