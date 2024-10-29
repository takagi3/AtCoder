use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: [String; n],
    }

    let mut ans: Vec<&str> = s.iter().take(k).map(String::as_str).collect();
    ans.sort();

    println!("{}", ans.join("\n"));
}
