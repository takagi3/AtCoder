use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [u32; n],
    }

    let ans = if n == 1 {
        0
    } else {
        let max_score = *p.iter().skip(1).max().unwrap();
        if max_score < p[0] { 0 } else { max_score - p[0] + 1 }
    };

    println!("{}", ans);
}
