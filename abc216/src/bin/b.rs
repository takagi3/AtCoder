use proconio::input;

fn main() {
    input! {
        n: usize,
        st: [(String, String); n],
    }

    let mut ans: &str = "No";
    for i in 0..n - 1 {
        for j in i + 1..n {
            if st[i].0 == st[j].0 && st[i].1 == st[j].1 {
                ans = "Yes"
            }
        }
    }

    println!("{}", ans);
}
