use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let cnt: usize = s.iter().filter(|&x| x == "For").count();
    let ans: &str = if cnt > n / 2 { "Yes" } else { "No" };

    println!("{}", ans);
}
