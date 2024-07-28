use proconio::input;

fn main() {
    input! {
        mut a: u32,
        b: u32,
        k: u32,
    }

    let mut ans: u32 = 0;
    for i in 0..10_u32.pow(9) {
        if a >= b {
            ans = i;
            break;
        }
        a *= k
    }

    println!("{}", ans);
}
