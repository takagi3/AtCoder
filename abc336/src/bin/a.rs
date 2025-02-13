use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let os = "o".repeat(n);
    let ans = format!("L{}ng", os);

    println!("{}", ans);
}