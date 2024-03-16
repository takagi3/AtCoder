use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        lrx: [(usize, usize, i32); q],
    }

    let mut b: Vec<i32> = vec![0; n + 1];
    for i in 0..q {
        b[lrx[i].0] += lrx[i].2;
        if lrx[i].1 < n {
            b[lrx[i].1 + 1] -= lrx[i].2;
        }
    }

    let mut ans: Vec<&str> = vec![""; n - 1];
    for i in 0..n - 1 {
        if b[i + 2] > 0 {
            ans[i] = "<";
        } else if b[i + 2] < 0 {
            ans[i] = ">";
        } else {
            ans[i] = "=";
        }
    }

    println!("{}", ans.iter()
        .map(std::string::ToString::to_string)
        .collect::<Vec<_>>()
        .join("")
    );
}
