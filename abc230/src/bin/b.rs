use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut ans: &str = "No";
    let t: &str = "oxxoxxoxxoxx";
    for i in 0..3 {
        if s == &t[i..i + s.len()] {
            ans = "Yes";
            break;
        }
    }

    println!("{}", ans);
}
