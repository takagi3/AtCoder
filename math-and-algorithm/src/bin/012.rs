use proconio::input;

fn main() {
    input! {
        n: u64,
    }

    let mut ans: &str = "Yes";
    for i in 2..=(n as f64).sqrt() as u64 {
        if n % i == 0 {
            ans = "No";
            break;
        }
    }

    println!("{}", ans);
}
