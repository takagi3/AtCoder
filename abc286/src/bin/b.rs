use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut ans: String = String::new();
    let mut i: usize = 0;
    while i < n {
        if i < n - 1 && s[i] == 'n' && s[i + 1] == 'a' {
            ans.push_str("nya");
            i += 2;
        } else {
            ans.push(s[i]);
            i += 1;
        }
    }

    println!("{}", ans);
}
