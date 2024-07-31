use proconio::input;

fn main() {
    input! {
        h: u32,
        w: u32,
        r: u32,
        c: u32,
    }

    let ans: u32;
    if h == 1 && w == 1 {
        ans = 0
    } else if h == 1 || w == 1 {
        if r == 1 && c == 1 || r == h && c == 1 || h == 1 && c == w {
            ans = 1
        } else {
            ans = 2
        }
    } else {
        if r == 1 && c == 1 || r == h && c == 1 || r == 1 && c == w || r == h && c == w {
            ans = 2
        } else if r == 1 || c == 1 || r == h || c == w {
            ans = 3
        } else {
            ans = 4
        }
    }

    println!("{}", ans);
}
