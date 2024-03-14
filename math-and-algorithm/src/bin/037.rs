use proconio::input;
use std::cmp::{max, min};

fn cross(ax: i64, ay: i64, bx: i64, by: i64) -> i64 {
    return ax * by - ay * bx;
}

fn main() {
    input! {
        x1: i64,
        y1: i64,
        x2: i64,
        y2: i64,
        x3: i64,
        y3: i64,
        x4: i64,
        y4: i64,
    }

    let ans1: i64 = cross(x2 - x1, y2 - y1, x3 - x1, y3 - y1);
    let ans2: i64 = cross(x2 - x1, y2 - y1, x4 - x1, y4 - y1);
    let ans3: i64 = cross(x4 - x3, y4 - y3, x1 - x3, y1 - y3);
    let ans4: i64 = cross(x4 - x3, y4 - y3, x2 - x3, y2 - y3);

    let ans: &str;
    if ans1 == 0 && ans2 == 0 && ans3 == 0 && ans4 == 0 {
        let mut a: (i64, i64) = (x1, y1);
        let mut b: (i64, i64) = (x2, y2);
        let mut c: (i64, i64) = (x3, y3);
        let mut d: (i64, i64) = (x4, y4);

        if a > b { (a, b) = (b, a) }
        if c > d { (c, d) = (d, c) }

        if max(a, c) <= min(b, d) {
            ans = "Yes";
        } else {
            ans = "No";
        }
    } else {
        let mut isab: bool = false;
        let mut iscd: bool = false;

        if ans1 >= 0 && ans2 <= 0 { isab = true }
        if ans1 <= 0 && ans2 >= 0 { isab = true }
        if ans3 >= 0 && ans4 <= 0 { iscd = true }
        if ans3 <= 0 && ans4 >= 0 { iscd = true }

        if isab && iscd {
            ans = "Yes";
        } else {
            ans = "No";
        }
    }

    println!("{}", ans);
}
