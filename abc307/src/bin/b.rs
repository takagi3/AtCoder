use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let mut ans = "No";
    'nest: for i in 0..n {
        for j in 0..n {
            if i != j {
                let base: Vec<char> = s[i].iter().chain(&s[j]).cloned().collect();
                if base.iter().eq(base.iter().rev()) {
                    ans = "Yes";
                    break 'nest;
                }
            }
        }
    }

    println!("{}", ans);
}
