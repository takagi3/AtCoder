use proconio::input;

fn main() {
    input! { n: usize, q: usize }
    let mut l = 1;
    let mut r = 2;
    let mut cnt = 0;
    for _ in 0..q {
        input! { dir: char, t: usize }
        let (pos, other) = if dir == 'L' { (&mut l, r) } else { (&mut r, l) };
        cnt += move_ptr(pos, other, t, n);
    }
    println!("{}", cnt);
}

fn move_ptr(pos: &mut usize, other: usize, target: usize, n: usize) -> usize {
    if *pos == target { return 0; }
    let orig = *pos;
    let mut cur;

    cur = orig;
    for step in 1..=n {
        cur = if cur == n { 1 } else { cur + 1 };
        if cur == other { break; }
        if cur == target {
            *pos = cur;
            return step;
        }
    }

    cur = orig;
    for step in 1..=n {
        cur = if cur == 1 { n } else { cur - 1 };
        if cur == other { break; }
        if cur == target {
            *pos = cur;
            return step;
        }
    }

    *pos = cur;
    0
}
