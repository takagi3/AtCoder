use proconio::input;

fn main() {
    input! {
        s: [String; 3],
    }

    let mut ans: Vec<&str> = vec!["ABC", "ARC", "AGC", "AHC"];
    for i in 0..3 {
        if let Some(idx) = ans.iter().position(|x| *x == s[i]) {
            ans.remove(idx);
        }
    }

    println!("{}", ans[0]);
}
