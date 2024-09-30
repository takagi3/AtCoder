use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }

    let mut ans: Vec<u32> = vec![0; w];
    for i in 0..w {
        for j in 0..h {
            if c[j][i] == '#' {
                ans[i] += 1
            }
        }
    }

    println!("{}", ans.iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" ")
    );
}
