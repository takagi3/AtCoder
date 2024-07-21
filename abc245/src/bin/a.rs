use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
        d: u32,
    }

    let mut ans: &str = "Aoki";
    if a < c {
        ans = "Takahashi"
    } else if a == c {
        if b <= d {
            ans = "Takahashi"
        }
    }

    println!("{}", ans);
}
