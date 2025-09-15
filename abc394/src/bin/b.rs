use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: [String; n],
    }

    s.sort_by_key(|t| t.len());
    let ans: String = s.into_iter().collect();

    println!("{}", ans);
}
