use proconio::input;

fn main() {
    input! {
        w: usize,
        b: usize,
    }

    let pattern = "wbwbwwbwbwbw";
    let repeat_count = (w + b) / pattern.len() + 2;
    let s = pattern.repeat(repeat_count);
    let mut ans: &str = "No";

    for slice in s.as_bytes().windows(w + b) {
        let (count_w, count_b) = slice.iter().fold((0, 0), |(cw, cb), &c| match c {
            b'w' => (cw + 1, cb),
            b'b' => (cw, cb + 1),
            _ => (cw, cb),
        });

        if count_w == w && count_b == b {
            ans = "Yes";
            break;
        }
    }

    println!("{}", ans);
}
