use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut ans: String = String::new();
    for _ in 0..6 / s.len() {
        ans.push_str(&s)
    }

    println!("{}", ans);
}
